use crate::CommandManager;
use serenity::{
    builder::CreateApplicationCommands,
    model::prelude::{
        command::{CommandOptionType, CommandType},
        interaction::application_command::ApplicationCommandInteraction,
    },
    prelude::Context,
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
                .name("xkcd")
                .description("Return a xkcd comic.")
                .kind(CommandType::ChatInput)
                .create_option(|option| {
                    option
                        .name("num")
                        .description("Tag to search; Random if empty.")
                        .kind(CommandOptionType::Integer)
                        .min_int_value(0)
                })
        })
    }

    async fn handler(
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> Result<bool, serenity::Error> {
        match command.data.name.as_str() {
            "xkcd" => xkcd(ctx, command).await?,
            _ => return Ok(false),
        }
        Ok(true)
    }
}

async fn xkcd(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    let num = command.data.options.get(0).map(|data| {
        data.value
            .as_ref()
            .map(|val| val.as_u64().unwrap())
            .unwrap()
    });

    let xkcd_query = match num {
        Some(val) => xkcd::get_xkcd(val).await,
        None => xkcd::get_xkcd_random().await,
    };

    command
        .create_interaction_response(ctx, |res| {
            res.interaction_response_data(|data| match xkcd_query {
                Ok(xkcd) => data.embed(|embed| {
                    embed
                        .description(format!(
                            "{} | {} | {}",
                            xkcd.title(),
                            xkcd.link(),
                            xkcd.num(),
                        ))
                        .image(xkcd.img())
                        .footer(|footer| {
                            footer.text(format!("{}/{}/{}", xkcd.year(), xkcd.month(), xkcd.day()))
                        })
                }),
                Err(why) => data.content(format!("Error: {:?}", why)),
            })
        })
        .await?;

    Ok(())
}
