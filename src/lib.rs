#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate hyper;

#[macro_use]
extern crate error_chain;

use hyper::Client;
use hyper::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

use std::io::Read;

pub mod errors;
use self::errors::*;

pub mod types;

pub struct Bot {
    base: String,
    client: Client,
    offset: i64
}

impl Bot {
    pub fn new(token: &str) -> Bot {
        Bot {
            base: format!("https://api.telegram.org/bot{}/", token),
            client: Client::new(),
            offset: 0
        }
    }
    fn send(&mut self, method: &str, body: &str) -> Result<serde_json::Value> {
        let mut res = self.client
            .post(&format!("{}{}", &self.base, method))
            .header(ContentType(Mime(TopLevel::Application,
                                     SubLevel::Json,
                                     vec![(Attr::Charset, Value::Utf8)])))
            .body(body)
            .send()
            .chain_err(|| format!("request for `{}` failed",method))?;
        let mut s = String::new();
        res.read_to_string(&mut s).chain_err(|| "cannot parse response")?;
        println!("{:?}",&s);
        let serialized : types::Response = serde_json::from_str(&s).chain_err(|| "cannot parse api response")?;
        match serialized.ok {
            true => Ok(serialized.result.unwrap()),
            false => bail!(ErrorKind::ApiError(serialized.description.unwrap()))
        }
    }
    pub fn get_chat(&mut self, chat_id: &str) -> Result<types::Chat> {
        #[derive(Serialize)]
        struct ChatId<'a> {
            chat_id: &'a str
        };
        let body = ChatId { chat_id : chat_id};
        let deserialized = serde_json::to_string(&body).unwrap();
        let resp = self.send("getChat",&deserialized)?;
        let serialized = serde_json::from_value(resp).chain_err(|| "cannot parse response of `getChat`")?;
        Ok(serialized)
    }
    pub fn get_me(&mut self) -> Result<types::User> {
        let resp = self.send("getMe","")?;
        let serialized = serde_json::from_value(resp).chain_err(|| "cannot parse response of `getMe`")?;
        Ok(serialized)
    }
    pub fn get_updates(&mut self) -> Result<Vec<types::Update>> {
        #[derive(Serialize)]
        struct GetUpdates {
            offset: i64,
        };
        let body = GetUpdates { offset : self.offset};
        let deserialized = serde_json::to_string(&body).unwrap();
        let resp = self.send("getUpdates",&deserialized)?;
        let serialized: Vec<types::Update> = serde_json::from_value(resp)
            .chain_err(|| "cannot parse response of `getUpdates`")?;
        self.offset = serialized.iter().fold(self.offset,|m,u| std::cmp::max(m,u.update_id)) + 1;
        Ok(serialized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut bot = Bot::new("232529554:AAG_xutLTVJvmzQ-pQp_6PNij_SCgE4uqCk");
        println!("{:?}", bot.get_me());
    }
}
