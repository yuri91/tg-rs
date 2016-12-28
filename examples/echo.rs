extern crate tg;
#[macro_use]
extern crate log;
extern crate env_logger;
use tg::HyperBot;
use tg::errors::*;
use tg::api::{Api,request,response};

fn main() {
    env_logger::init().unwrap();
    let mut bot = HyperBot::new("232529554:AAG_xutLTVJvmzQ-pQp_6PNij_SCgE4uqCk");
    loop {
        let updates = bot.get_updates(50).expect("get_updates failed");
        for u in updates {
            info!("update: {:?}",&u);
            match u.content {
                response::UpdateKind::Message(rcv) => {
                    let send = request::Message {
                        chat_id: rcv.from.unwrap().id,
                        text: &rcv.text.unwrap(),
                        reply_to_message_id: Some(rcv.message_id),
                        .. Default::default()
                    };
                    info!("send_message request: {:?}",&send);
                    info!("send_message response: {:?}",&bot.send_message(&send));
                },
                response::UpdateKind::InlineQuery(i) => {
                    let a = request::InlineQueryAnswer {
                        inline_query_id: &i.id,
                        results: vec![request::InlineQueryResult::Article{
                            title: "much choice",
                            id: "1",
                            input_message_content: request::InputMessageContent::Text {
                                message_text: "rust rulez"
                            }
                        }],
                    };
                    bot.answer_inline_query(&a);
                },
                _ => println!("unsupported update kind: {:?}",&u.content)
            }
        }
    }
}
