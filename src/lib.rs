#![feature(proc_macro)]
#![feature(conservative_impl_trait)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

extern crate hyper;

#[macro_use]
extern crate error_chain;

use hyper::Client;
use hyper::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

use serde::ser::Serialize;
use serde::de::Deserialize;

use std::fmt::Debug;
use std::io::Read;
use std::time::Duration;

pub mod errors;
use self::errors::*;

pub mod api;
use api::{request,response};

pub struct HyperBot {
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


impl HyperBot {
    pub fn new(token: &str) -> HyperBot {
        let mut bot = HyperBot {
            base: format!("https://api.telegram.org/bot{}/", token),
            client: Client::new(),
            offset: 0,
            timeout: Duration::from_secs(120)
        };
        bot.client.set_read_timeout(Some(bot.timeout));
        bot
    }
}
impl api::Client for HyperBot {
    fn set_updates_offset(&mut self, offset: u64) { self.offset = offset; }
    fn get_updates_offset(&self) -> u64 { self.offset }

    fn send<S: Serialize+Debug,D: Deserialize>(&mut self, method: &str, body: &S) -> Result<D> {
        let serialized = serde_json::to_string(body).unwrap();
        let mut res = self.client
            .post(&format!("{}{}", &self.base, method))
            .header(ContentType(Mime(TopLevel::Application,
                                     SubLevel::Json,
                                     vec![(Attr::Charset, Value::Utf8)])))
            .body(&serialized)
            .send()
            .chain_err(|| format!("request for `{}` failed",method))?;
        let mut s = String::new();
        debug!("[Bot::send] Sending `{}`: {:?}",method, body);
        res.read_to_string(&mut s).chain_err(|| "cannot parse response")?;
        debug!("[Bot::send] Received {}",&s);
        let deserialized : Response = serde_json::from_str(&s).chain_err(|| "cannot parse api response")?;
        debug!("[Bot::send] Parsed {:?}",&deserialized);
        match deserialized.ok {
            true => {
                let response = serde_json::from_value(deserialized.result.unwrap())
                    .chain_err(|| format!("cannot parse result of `{}`",method))?;
                Ok(response)
            },
            false => bail!(ErrorKind::ApiError(deserialized.description.unwrap()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::api::Client;
    #[test]
    fn it_works() {
        let mut bot = HyperBot::new("232529554:AAG_xutLTVJvmzQ-pQp_6PNij_SCgE4uqCk");
        println!("{:?}", bot.get_me());
    }
}
