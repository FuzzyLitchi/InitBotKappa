use discord::Discord;
use discord::Connection;
use discord::model::Message;

static PREFIX: &str = "!";

pub struct Bot {
    discord: Discord,
    pub connection: Connection,
}

impl Bot {
    pub fn from_bot_token(bot_token: &str) -> Bot {
        let discord = Discord::from_bot_token(bot_token).expect("login failed");
        let (mut connection, _) = discord.connect().expect("connect failed");

        Bot {
            discord,
            connection,
        }
    }

    pub fn handle_message(&self, message: Message) {
        if message.author.bot {
            return
        }

        if !message.content.starts_with(PREFIX) {
            return
        }

        let text = message.content.clone().split_off(PREFIX.len());

        let (command, args) = {
            let mut vector: Vec<String> = text.split(' ').map(|s| s.to_owned()).collect();

            (vector.remove(0), vector)
        };

        //run command
    }
}
