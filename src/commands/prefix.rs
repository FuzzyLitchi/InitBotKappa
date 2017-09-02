use bot;
use discord::model::Message;

pub fn prefix(bot: &mut bot::Bot, message: &Message, args: &Vec<String>) {
    if let Some(prefix) = args.get(0) {
        bot.prefix = prefix.clone();
        bot.discord.send_message(message.channel_id, &format!("The prefix has been changed to \"{}\".", prefix), "", false)
                   .expect("Failed to send message");
    } else {
        bot.discord.send_message(message.channel_id, "Please specify a prefix.", "", false)
                   .expect("Failed to send message");
    }
}
