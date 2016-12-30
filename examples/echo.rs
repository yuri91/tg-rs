extern crate tg;
#[macro_use]
extern crate log;
extern crate env_logger;
use tg::HyperBot;
use tg::api::{Api,request,response};

use std::fs::File;
use std::io::Read;

fn main() {
    env_logger::init().unwrap();

    let mut f = File::open("TOKEN").expect("missing TOKEN file");
    let mut token = String::new();
    f.read_to_string(&mut token).expect("cannot read TOKEN file");
    let mut bot = HyperBot::new(&token);
    loop {
        let updates = bot.get_updates(50).expect("get_updates failed");
        for u in updates {
            info!("update: {:?}",&u);
            match u.content {
                response::UpdateKind::Message(rcv) => {
                    let send = request::Message::new(rcv.from.unwrap().id,rcv.text.unwrap())
                        .with_reply_to_message_id(rcv.message_id);
                    info!("send_message request: {:?}",&send);
                    info!("send_message response: {:?}",&bot.send_message(&send));
                },
                response::UpdateKind::InlineQuery(i) => {
                    let a = request::InlineQueryAnswer::new(&i.id,
                        vec![request::InlineQueryResult::Article{
                            title: "much choice",
                            id: "1",
                            input_message_content: request::InputMessageContent::text("rust rulez")
                        }],
                    );
                    info!("answer_inline_query response: {:?}",&bot.answer_inline_query(&a));
                },
                _ => println!("unsupported update kind: {:?}",&u.content)
            }
        }
    }
}
