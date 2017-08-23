use discord::Discord;
use discord::model::Message;

static prefix: &str = "!";

struct Bot {
    discord: Discord,
}

impl Bot {
    pub fn from_bot_token(bot_token: &str) -> Bot {
        Bot {
            discord: Discord::from_bot_token(bot_token).expect("login failed"),
        }
    }

    fn handle_message(&self, message: Message) {
        if message.author.bot {
            return
        }

        if message.content.starts_with(prefix) {
            let text = message.content.clone().split_off(prefix.len());

            let (command, args) = {
                let mut vector: Vec<String> = text.split(' ').map(|s| s.to_owned()).collect();

                (vector.pop().unwrap(), vector)
            };
        }
    }
}
