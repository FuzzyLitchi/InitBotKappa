extern crate discord;

mod bot;

use bot::Bot;
use std::env;
use discord::model::Event;

fn main() {
    let mut bot = Bot::from_bot_token(
        &env::var("DISCORD_TOKEN").expect("Expected token"),
    );

    println!("Ready.");

    loop {
        match bot.connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                    bot.handle_message(message);
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break
            }
            Err(err) => println!("Receive error: {:?}", err)
        }
    }
}
