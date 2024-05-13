use serenity::builder::*;


pub fn register() -> CreateCommand {
    CreateCommand::new("create_message")
        .description("Не вызывать, только для настройки")
}
