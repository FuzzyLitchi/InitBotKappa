use commands::*;
use discord;
use discord::Discord;
use discord::Connection;
use discord::model::Event;
use discord::model::Message;

pub struct Bot {
    pub discord: Discord,
    connection: Connection,
    pub prefix: String,
}

impl Bot {
    pub fn from_bot_token(bot_token: &str) -> Bot {
        let discord = Discord::from_bot_token(bot_token).expect("login failed");
        let (connection, _) = discord.connect().expect("connect failed");

        Bot {
            discord,
            connection,
            prefix: String::from("!"),
        }
    }

    pub fn serve_forever(&mut self) -> ! {
        loop {
            match self.connection.recv_event() {
                Ok(Event::MessageCreate(message)) => {
                        self.handle_message(message);
                }
                Ok(_) => {}
                Err(discord::Error::Closed(code, body)) => {
                    panic!("Gateway closed on us with code {:?}: {}", code, body);
                }
                Err(err) => println!("Receive error: {:?}", err)
            }
        }
    }

    fn handle_message(&mut self, message: Message) {
        if message.author.bot {
            return
        }

        if !message.content.starts_with(&self.prefix) {
            return
        }

        let text = message.content.clone().split_off(self.prefix.len());

        let (command, args) = {
            let mut vector: Vec<String> = text.split_whitespace().map(|s| s.to_owned()).collect();

            (vector.remove(0), vector)
        };

        match command.as_ref() {
            "ping" => ping::ping(self, &message),
            "prefix" => prefix::prefix(self, &message, &args),
            _ => (),
        }
    }
}
