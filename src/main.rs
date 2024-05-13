use reqwest::Url;
use serenity::all::{ActivityData, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, GuildId, Interaction};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

pub mod config;
pub mod commands;


async fn send_to_telegram(msg: &str) {
    let base_url = format!("https://api.telegram.org/bot{}/sendMessage", config::CONFIG.telegram_bot_token);

    let url = Url::parse_with_params(
        base_url.as_ref(),
        &[
            ("chat_id", &config::CONFIG.telegram_channel_id.to_string().as_ref()),
            ("text", &msg)
        ]
    ).unwrap();

    reqwest::get(url).await.expect("Error sending message to Telegram");
}


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "create_message" => Some("#init".to_string()),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        } else if let Interaction::Autocomplete(interaction) = interaction {
            // println!("Received autocomplete interaction: {interaction:#?}");

            // let content = match interaction.data.name.as_str() {
            //     "game" => {
            //         let games = vec!["Dota 2", "CS:GO", "PUBG"];
            //         let options = games.iter().map(|game| {
            //             CreateCommandOptionChoice::new(game, game)
            //         }).collect();

            //         let data = CreateInteractionResponseMessage::new().content("Выберите игру").add_option(
            //             CreateCommandOption::new(
            //                 CommandOptionType::String, "game", "Игра"
            //             )
            //             .required(true)
            //             .set_autocomplete(true)
            //             .add_choices(options)
            //         );

            //         Some(data)
            //     },
            //     _ => None,
            // };

            // if let Some(content) = content {
            //     let builder = CreateInteractionResponse::Message(content);
            //     if let Err(why) = interaction.create_response(&ctx.http, builder).await {
            //         println!("Cannot respond to autocomplete command: {why}");
            //     }
            // }
        }
    }

    async fn message(&self, _ctx: Context, msg: Message) {
        if msg.guild_id != Some(config::CONFIG.discord_guild_id.into()) {
            return;
        }

        if msg.channel_id == config::CONFIG.discord_channel_id {
            send_to_telegram(&msg.content).await;
            return;
        }
    }

    async fn ready(&self, ctx: Context, _ready: serenity::model::gateway::Ready) {
        let guild_id = GuildId::new(config::CONFIG.discord_guild_id);

        let _ = guild_id
            .set_commands(
                &ctx.http,
                vec![
                    commands::add_game::register(),
                    commands::delete_game::register(),
                    commands::create_message::register()
                ]
            ).await.unwrap();
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
