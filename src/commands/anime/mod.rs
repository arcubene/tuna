mod quote;
mod waifu;

use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        interaction::{
            application_command::ApplicationCommandInteraction,
            autocomplete::AutocompleteInteraction,
        },
        *,
    },
    prelude::*,
};

pub(crate) fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("anime")
        .description("Anime commands.")
        .kind(command::CommandType::ChatInput)
        .create_option(quote::register)
        .create_option(waifu::register)
}

pub(crate) async fn anime(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    let options = interaction.data.options.as_slice();

    match options.get(0).expect("Option").name.as_str() {
        "quote" => quote::quote(ctx, interaction).await,
        "waifu" => waifu::waifu(ctx, interaction).await,
        _ => unreachable!(),
    }
}

pub(crate) async fn anime_autocomplete(
    ctx: &Context,
    autocomplete: &AutocompleteInteraction,
) -> Result<(), serenity::Error> {
    let options = autocomplete.data.options.as_slice();

    match options.get(0).expect("Option").name.as_str() {
        "waifu" => waifu::waifu_autocomplete(ctx, autocomplete).await,
        _ => unreachable!(),
    }
}
