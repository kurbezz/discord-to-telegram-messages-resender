use serenity::builder::*;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::CreateQuickModal;


pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let modal = CreateQuickModal::new("Добавление игры")
        .timeout(std::time::Duration::from_secs(600))
        .short_field("Категория")
        .paragraph_field("Игра");
    let response = interaction.quick_modal(ctx, modal).await?.unwrap();

    let inputs = response.inputs;
    let (category, game) = (&inputs[0], &inputs[1]);

    response
        .interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                format!("{} {}", category, game),
            )),
        )
        .await?;
    Ok(())
}


pub fn register() -> CreateCommand {
    CreateCommand::new("add_game").description("Добавить игру в список")
}
