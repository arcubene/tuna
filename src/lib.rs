mod commands;

use commands::*;
use serenity::{
    client::bridge::gateway::ShardManager,
    model::prelude::{command::Command, interaction::Interaction},
};
use serenity::{model::gateway::Ready, prelude::*};
use shuttle_secrets::SecretStore;

struct TunaBot;

#[serenity::async_trait]
impl EventHandler for TunaBot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        log::info!("{} is connected!", ready.user.name);

        let ctx = std::sync::Arc::new(ctx);

        Command::set_global_application_commands(&ctx.http, |commands| {
            dice::Commands::register(ctx.clone(), commands);
            anime::Commands::register(ctx.clone(), commands);
            xkcd::Commands::register(ctx.clone(), commands);
            commands
        })
        .await
        .unwrap();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::ApplicationCommand(command) => {
                log::info!("Responded");

                // If any of the handler functions returns true, the or operator short-circuits.
                let result = dice::Commands::handler(&ctx, &command).await.unwrap()
                    || anime::Commands::handler(&ctx, &command).await.unwrap()
                    || xkcd::Commands::handler(&ctx, &command).await.unwrap();

                if !result {
                    unreachable!("Command not handled: {:?}", command);
                }
            }
            Interaction::Autocomplete(autocomplete) => {
                // If any of the handler functions returns true, the or operator short-circuits.
                let result = anime::Commands::autocomplete_handler(&ctx, &autocomplete)
                    .await
                    .unwrap();

                if !result {
                    unreachable!("Command not handled: {:?}", autocomplete);
                }
            }
            _ => unreachable!("Interaction not handled: {:?}", interaction),
        }
    }

    async fn message(&self, ctx: Context, new_message: serenity::model::prelude::Message) {
        if new_message.content.to_lowercase() == "!kill" {
            let owner_id = ctx
                .data
                .read()
                .await
                .get::<crate::BotOwner>()
                .copied()
                .unwrap_or_default();

            if new_message.author.id == owner_id {
                let shard_manager = ctx
                    .data
                    .read()
                    .await
                    .get::<crate::ShardManagerContainer>()
                    .unwrap()
                    .clone();

                new_message
                    .reply_ping(ctx, "oh, i'm die. thank you forever.")
                    .await
                    .unwrap();

                shard_manager.lock().await.shutdown_all().await;
            }
        }
    }
}

#[shuttle_service::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml` from the shared Postgres database
    let token = secret_store.get("DISCORD_TOKEN").expect("Discord Token");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::all();

    let client = Client::builder(&token, intents)
        .event_handler(TunaBot)
        .await
        .expect("Err creating client");

    // Owner user id for owner only commands.
    let bot_owner = serenity::model::prelude::UserId::from(
        secret_store
            .get("OWNER_ID")
            .expect("Owner Id")
            .parse::<u64>()
            .unwrap(),
    );

    let mut data = client.data.write().await;
    data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    data.insert::<BotOwner>(bot_owner);
    drop(data);

    Ok(client)
}

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = std::sync::Arc<Mutex<ShardManager>>;
}

pub struct BotOwner;

impl TypeMapKey for BotOwner {
    type Value = serenity::model::prelude::UserId;
}
