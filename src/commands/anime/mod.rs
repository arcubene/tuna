mod quote;
mod waifu;

use super::CommandManager;
use serenity::{
    builder::CreateApplicationCommands,
    model::prelude::{
        command::{CommandOptionType, CommandType},
        interaction::{
            application_command::ApplicationCommandInteraction,
            autocomplete::AutocompleteInteraction,
        },
    },
    prelude::*,
};

pub struct Commands;

#[serenity::async_trait]
impl CommandManager for Commands {
    fn register(
        _: std::sync::Arc<Context>,
        commands: &mut CreateApplicationCommands,
    ) -> &mut CreateApplicationCommands {
        commands.create_application_command(|command| {
            command
                .name("anime")
                .description("Anime commands.")
                .kind(CommandType::ChatInput)
                .create_option(|option| {
                    option
                        .name("waifu")
                        .description("Return a waifu image or gif.")
                        .kind(CommandOptionType::SubCommand)
                        .create_sub_option(|sub_option| {
                            sub_option
                                .name("tag")
                                .description("Tag to search; Random if empty.")
                                .kind(CommandOptionType::String)
                                .set_autocomplete(true)
                        })
                })
                .create_option(|option| {
                    option
                        .name("quote")
                        .description("Return an anime quote.")
                        .kind(CommandOptionType::SubCommand)
                })
        })
    }

    async fn handler(
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> Result<bool, serenity::Error> {
        match command.data.name.as_str() {
            "anime" => match command.data.options.get(0).unwrap().name.as_str() {
                "waifu" => waifu::waifu(ctx, command).await?,
                "quote" => quote::quote(ctx, command).await?,
                _ => return Ok(false),
            },
            _ => return Ok(false),
        }
        Ok(true)
    }

    async fn autocomplete_handler(
        ctx: &Context,
        autocomplete: &AutocompleteInteraction,
    ) -> Result<bool, serenity::Error> {
        match autocomplete.data.name.as_str() {
            "anime" => match autocomplete.data.options.get(0).unwrap().name.as_str() {
                "waifu" => waifu::waifu_autocomplete(ctx, autocomplete).await?,
                _ => return Ok(false),
            },
            _ => return Ok(false),
        }
        Ok(true)
    }
}
