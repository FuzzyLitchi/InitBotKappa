extern crate discord;

mod bot;

use bot::Bot;
use std::env;

fn main() {
    let bot = Bot::from_bot_token(
        &env::var("DISCORD_TOKEN").expect("Expected token"),
    );

    println!("Ready.");

    /*loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                println!("{} says: {}", message.author.name, message.content);
                if message.author.name == "FuzzyLitchi" {
                    let _ = discord.send_message(message.channel_id, message.content.as_str(), "", false);
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break
            }
            Err(err) => println!("Receive error: {:?}", err)
        }
    }*/
}
