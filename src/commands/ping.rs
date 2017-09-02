use bot;
use discord::model::Message;

pub fn ping(bot: &bot::Bot, message: &Message) {
    bot.discord.send_message(message.channel_id, "Pong!", "", false)
               .expect("Failed to send message");
}
