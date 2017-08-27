extern crate discord;

mod commands;
mod bot;

use bot::Bot;
use std::env;

fn main() {
    let mut bot = Bot::from_bot_token(
        &env::var("DISCORD_TOKEN").expect("Expected token"),
    );

    bot.serve_forever();
}
