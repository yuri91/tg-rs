pub mod request;
pub mod response;

use serde::ser::Serialize;
use serde::de::Deserialize;
use std::fmt::Debug;
use std::cmp::{max,min};
use super::errors::*;

pub trait Api {
    fn send<S: Serialize+Debug,D: Deserialize>(&mut self, method: &str, body: &S) -> Result<D>;
    fn set_updates_offset(&mut self, offset: u64);
    fn get_updates_offset(&self) -> u64;

    fn get_chat(&mut self, chat_id: u64) -> Result<response::Chat> {
        let body = request::Chat { chat_id : chat_id};
        self.send("getChat",&body)
    }
    fn get_me(&mut self) -> Result<response::User> {
        self.send("getMe",&request::Empty{})
    }
    fn get_updates(&mut self, timeout: u32) -> Result<Vec<response::Update>> {
        let body = request::Update { offset: self.get_updates_offset(), timeout: min(100,timeout) as i32};
        let mut resp : Vec<response::Update> = self.send("getUpdates",&body)?;
        let new_offset = resp.iter().fold(self.get_updates_offset(),|m,u| max(m,u.update_id)) + 1;
        // Telegram sometimes sends already acknowleged messages
        // A workaround seems to be to reset the offset whenever there are no
        // new messages in the update
        let size_before = resp.len();
        resp.retain(|u| u.update_id >= self.get_updates_offset());
        let size_after = resp.len();
        if size_after < size_before {
            debug!("[Bot::get_updates] Got {} duplicates!",size_before-size_after);
        }
        self.set_updates_offset(new_offset);
        if size_after==0 {
            debug!("[Bot::get_updates] Zero updates! resetting offset...");
            self.set_updates_offset(0);
        }
        Ok(resp)
    }
    fn send_message(&mut self, msg: &request::Message) -> Result<response::Message> {
        self.send("sendMessage",msg)
    }
    fn answer_inline_query(&mut self, answer: &request::InlineQueryAnswer) -> Result<bool> {
        self.send("answerInlineQuery",answer)
    }
}
