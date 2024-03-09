use serenity::all::ActivityData;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

pub mod config;


async fn send_to_telegram(msg: &str) {
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
        config::CONFIG.telegram_bot_token,
        config::CONFIG.telgram_channel_id,
        msg
    );

    reqwest::get(&url).await.expect("Error sending message to Telegram");
}


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
        if msg.channel_id != config::CONFIG.discord_channel_id {
            return;
        }

        send_to_telegram(&msg.content).await;
    }
}


#[tokio::main]
async fn main() {
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&config::CONFIG.discord_bot_token, intents)
            .event_handler(Handler)
            .status(serenity::all::OnlineStatus::Online)
            .activity(ActivityData::playing(&config::CONFIG.discord_bot_activity))
            .await
            .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
