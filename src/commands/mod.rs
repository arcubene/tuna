pub(super) mod anime;
pub(super) mod dice;
pub(super) mod xkcd;

use serenity::model::prelude::*;

#[serenity::async_trait]
pub(super) trait CommandManager {
    fn register(
        ctx: std::sync::Arc<serenity::prelude::Context>,
        commands: &mut serenity::builder::CreateApplicationCommands,
    ) -> &mut serenity::builder::CreateApplicationCommands;

    /// Returns a result where it is true if the command is handled.
    async fn handler(
        ctx: &serenity::prelude::Context,
        command: &prelude::interaction::application_command::ApplicationCommandInteraction,
    ) -> Result<bool, serenity::Error>;

    async fn autocomplete_handler(
        _ctx: &serenity::prelude::Context,
        _autocomplete: &prelude::interaction::autocomplete::AutocompleteInteraction,
    ) -> Result<bool, serenity::Error> {
        unimplemented!()
    }
}
