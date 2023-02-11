use serenity::{
    builder::CreateApplicationCommandOption,
    model::prelude::{interaction::application_command::ApplicationCommandInteraction, *},
    prelude::*,
};

pub(super) fn register(
    option: &mut CreateApplicationCommandOption,
) -> &mut CreateApplicationCommandOption {
    option
        .name("quote")
        .description("Return an anime quote.")
        .kind(command::CommandOptionType::SubCommand)
}

pub(super) async fn quote(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    let quote = animechan::get_quote_random().await.expect("Quote");

    interaction
        .create_interaction_response(ctx, |res| {
            res.interaction_response_data(|data| {
                data.embed(|embed| {
                    embed
                        .description(format!("\"{}\"", quote.quote()))
                        .footer(|footer| {
                            footer.text(format!("- {}, {}", quote.character(), quote.anime()))
                        })
                })
            })
        })
        .await
}
