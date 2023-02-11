use crate::commands;
use serenity::{
    model::{application::interaction::Interaction, prelude::*},
    prelude::*,
};
use tracing::log::*;

pub(crate) struct Bot {
    owner_id: UserId,
}

impl Bot {
    pub fn new(secret_store: shuttle_secrets::SecretStore) -> Self {
        // Get the developer discord id set in `Secrets.toml`
        let owner_id = UserId(
            secret_store
                .get("OWNER_ID")
                .expect("OWNER_ID")
                .parse()
                .expect("u64"),
        );

        Self { owner_id }
    }

    async fn kill(&self, ctx: Context) {
        let shard_manager = ctx
            .data
            .read()
            .await
            .get::<crate::ShardManagerContainer>()
            .expect("Shard Manager")
            .clone();

        shard_manager.lock().await.shutdown_all().await;
    }
}

#[serenity::async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        let Some(content) = msg.content.strip_prefix('?') else {
			return;
		};

        if let ("kill", true) = (
            content.to_lowercase().as_str(),
            msg.author.id == self.owner_id,
        ) {
            msg.reply_ping(&ctx, "oh, i'm die. thank you forever.")
                .await
                .expect("Message");

            self.kill(ctx).await;
        };
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        command::Command::set_global_application_commands(ctx, |commands| {
            commands
                .create_application_command(commands::anime::register)
                .create_application_command(commands::dice::register)
                .create_application_command(commands::xkcd::register);
            commands
        })
        .await
        .expect("Commands");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::ApplicationCommand(interaction) => {
                debug!(
                    "{:?} {:?} {:?} Application Command: {} {:?}",
                    interaction.guild_id,
                    interaction.channel_id,
                    interaction.user.id,
                    interaction.data.name,
                    interaction.data.options
                );

                match interaction.data.name.as_str() {
                    "anime" => commands::anime::anime(&ctx, &interaction).await,
                    "roll" => commands::dice::roll(&ctx, &interaction).await,
                    "xkcd" => commands::xkcd::xkcd(&ctx, &interaction).await,
                    _ => unreachable!(),
                }
                .expect("Application Command Response")
            }

            Interaction::Autocomplete(autocomplete) => {
                trace!(
                    "{:?} {:?} {:?} Autocomplete: {} {:#?}",
                    autocomplete.guild_id,
                    autocomplete.channel_id,
                    autocomplete.user.id,
                    autocomplete.data.name,
                    autocomplete.data.options
                );

                match autocomplete.data.name.as_str() {
                    "anime" => commands::anime::anime_autocomplete(&ctx, &autocomplete).await,
                    _ => unreachable!(),
                }
                .expect("Autocomplete Response")
            }

            _ => unreachable!(),
        }
    }
}
