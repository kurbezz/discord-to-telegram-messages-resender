use once_cell::sync::Lazy;


fn get_env(env: &'static str) -> String {
    std::env::var(env).unwrap_or_else(|_| panic!("Cannot get the {} env variable", env))
}


pub struct Config {
    pub discord_bot_token: String,
    pub discord_guild_id: u64,
    pub discord_channel_id: u64,
    pub discord_bot_activity: String,
    pub telegram_bot_token: String,
    pub telegram_channel_id: i128,
}


impl Config {
    pub fn load() -> Config {
        Config {
            discord_bot_token: get_env("DISCORD_BOT_TOKEN"),
            discord_guild_id: get_env("DISCORD_GUILD_ID").parse().unwrap(),
            discord_channel_id: get_env("DISCORD_CHANNEL_ID").parse().unwrap(),
            discord_bot_activity: get_env("DISCORD_BOT_ACTIVITY"),
            telegram_bot_token: get_env("TELEGRAM_BOT_TOKEN"),
            telegram_channel_id: get_env("TELEGRAM_CHANNEL_ID").parse().unwrap(),
        }
    }
}


pub static CONFIG: Lazy<Config> = Lazy::new(Config::load);
