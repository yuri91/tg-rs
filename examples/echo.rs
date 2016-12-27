extern crate tg;
use tg::Bot;
use tg::errors::*;
use tg::types::{request,response};

fn main() {
    let mut bot = Bot::new("232529554:AAG_xutLTVJvmzQ-pQp_6PNij_SCgE4uqCk");
    loop {
        let updates = bot.get_updates(50).expect("get_updates failed");
        for u in updates {
            println!("==========================");
            println!("update: {:?}",&u);
            if let Some(rcv) = u.message {
                let send = request::Message {
                    chat_id: rcv.from.unwrap().id,
                    text: &rcv.text.unwrap(),
                    reply_to_message_id: Some(rcv.message_id),
                    .. Default::default()
                };
                println!("send_message request: {:?}",&send);
                println!("send_message response: {:?}",&bot.send_message(&send));
                println!("==========================");
            }
        }
    }
}
