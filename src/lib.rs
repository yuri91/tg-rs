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

use std::time::Duration;

pub mod errors;
use self::errors::*;

pub mod types;
use types::{request,response};

pub struct Bot {
    base: String,
    client: Client,
    offset: u64,
    timeout: Duration
}
#[derive(Clone,Debug,Deserialize)]
struct Response {
    ok: bool,
    result: Option<serde_json::Value>,
    error_code: Option<u16>,
    description: Option<String>
}


impl Bot {
    pub fn new(token: &str) -> Bot {
        let mut bot = Bot {
            base: format!("https://api.telegram.org/bot{}/", token),
            client: Client::new(),
            offset: 0,
            timeout: Duration::from_secs(120)
        };
        bot.client.set_read_timeout(Some(bot.timeout));
        bot
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
        let serialized : Response = serde_json::from_str(&s).chain_err(|| "cannot parse api response")?;
        match serialized.ok {
            true => Ok(serialized.result.unwrap()),
            false => bail!(ErrorKind::ApiError(serialized.description.unwrap()))
        }
    }
    pub fn get_chat(&mut self, chat_id: u64) -> Result<response::Chat> {
        let body = request::Chat { chat_id : chat_id};
        let deserialized = serde_json::to_string(&body).unwrap();
        let resp = self.send("getChat",&deserialized)?;
        let serialized = serde_json::from_value(resp).chain_err(|| "cannot parse response of `getChat`")?;
        Ok(serialized)
    }
    pub fn get_me(&mut self) -> Result<response::User> {
        let resp = self.send("getMe","")?;
        let serialized = serde_json::from_value(resp).chain_err(|| "cannot parse response of `getMe`")?;
        Ok(serialized)
    }
    pub fn get_updates(&mut self, timeout: u32) -> Result<Vec<response::Update>> {
        let body = request::Update { offset: self.offset, timeout: std::cmp::min(100,timeout) as i32};
        let deserialized = serde_json::to_string(&body).unwrap();
        let resp = self.send("getUpdates",&deserialized)?;
        let mut serialized: Vec<response::Update> = serde_json::from_value(resp)
            .chain_err(|| "cannot parse response of `getUpdates`")?;
        let new_offset = serialized.iter().fold(self.offset,|m,u| std::cmp::max(m,u.update_id)) + 1;
        // Telegram sometimes sends already acknowleged messages
        // A workaround seems to be to reset the offset whenever there are no
        // new messages in the update
        serialized.retain(|u| u.update_id >= self.offset);
        self.offset = new_offset;
        if serialized.len()==0 {
            self.offset=0;
        }
        Ok(serialized)
    }
    pub fn send_message(&mut self, msg: &request::Message) -> Result<response::Message> {
        let deserialized = serde_json::to_string(msg).unwrap();
        let resp = self.send("sendMessage",&deserialized)?;
        let serialized = serde_json::from_value(resp).chain_err(|| "cannot parse response of `sendMessage`")?;
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
