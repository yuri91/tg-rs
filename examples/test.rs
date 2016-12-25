extern crate tg;
use tg::Bot;
use tg::errors::*;

fn test_method<T, F>(mut f: F) 
    where T: std::fmt::Debug, F: FnMut()->Result<T> {
    match f() {
        Ok(data) => {
            println!("{:?}",&data);
        },
        Err(ref e) => {
            println!("error: {}", e);
            for e in e.iter().skip(1) {
                println!("caused by: {}", e);
            }
            // The backtrace is not always generated. Try to run this example
            // with `RUST_BACKTRACE=1`.
            if let Some(backtrace) = e.backtrace() {
                println!("backtrace: {:?}", backtrace);
            }
            ::std::process::exit(1);
        }
    }
}

fn main() {
    let mut bot = Bot::new("232529554:AAG_xutLTVJvmzQ-pQp_6PNij_SCgE4uqCk");
    test_method(|| bot.get_me());
    test_method(|| bot.get_chat("57333322"));
    test_method(|| bot.get_updates());
    test_method(|| bot.get_updates());
}
