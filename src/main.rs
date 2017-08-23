extern crate discord;

use discord::Discord;
use discord::model::Event;
use std::env;

fn main() {
    let discord = Discord::from_bot_token(
        &env::var("DISCORD_TOKEN").expect("Expected token"),
    ).expect("login failed");

    let (mut connection, _) = discord.connect().expect("connect failed");

    println!("Ready.");

    loop {
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
    }
}
