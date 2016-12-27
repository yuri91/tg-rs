extern crate tg;
#[macro_use]
extern crate log;
extern crate env_logger;
use tg::HyperBot;
use tg::errors::*;
use tg::api::{Client,request,response};

fn main() {
    env_logger::init().unwrap();
    let mut bot = HyperBot::new("232529554:AAG_xutLTVJvmzQ-pQp_6PNij_SCgE4uqCk");
    loop {
        let updates = bot.get_updates(50).expect("get_updates failed");
        for u in updates {
            info!("update: {:?}",&u);
            if let Some(rcv) = u.message {
                let send = request::Message {
                    chat_id: rcv.from.unwrap().id,
                    text: &rcv.text.unwrap(),
                    reply_to_message_id: Some(rcv.message_id),
                    .. Default::default()
                };
                info!("send_message request: {:?}",&send);
                info!("send_message response: {:?}",&bot.send_message(&send));
            }
        }
    }
}
