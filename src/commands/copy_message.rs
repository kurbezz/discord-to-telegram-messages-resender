use serenity::{all::CommandOptionType, builder::*};


pub fn register() -> CreateCommand {
    CreateCommand::new("copy_message")
        .description("Не вызывать, только для настройки")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer, "message_id", "ID сообщения"
            )
            .required(true)
        )
}
