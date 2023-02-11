mod bot;
mod commands;

use bot::Bot;
use serenity::prelude::*;

#[shuttle_service::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
) -> shuttle_service::ShuttleSerenity {
    // Ensure all panics are logged
    std::panic::set_hook(Box::new(|why| tracing::log::error!("{why}")));

    // Get the discord token set in `Secrets.toml`
    let token = secret_store.get("DISCORD_TOKEN").expect("DISCORD_TOKEN");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::all();

    let bot = Bot::new(secret_store);

    let client = Client::builder(token, intents)
        .event_handler(bot)
        .await
        .expect("Err creating client");

    let mut data_writer = client.data.write().await;
    data_writer.insert::<ShardManagerContainer>(client.shard_manager.clone());
    drop(data_writer);

    Ok(client)
}

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = std::sync::Arc<Mutex<serenity::client::bridge::gateway::ShardManager>>;
}
