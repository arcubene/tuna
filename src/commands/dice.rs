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
                .name("roll")
                .description("Roll dice.")
                .kind(CommandType::ChatInput)
                .create_option(|option| {
                    option
                        .name("sides")
                        .description("Sides of the dice.")
                        .kind(CommandOptionType::Integer)
                        .min_int_value(2)
                        .max_int_value(120)
                })
                .create_option(|option| {
                    option
                        .name("num")
                        .description("Number of dice.")
                        .kind(CommandOptionType::Integer)
                        .min_int_value(1)
                        .max_int_value(100)
                })
        })
    }

    async fn handler(
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> Result<bool, serenity::Error> {
        match command.data.name.as_str() {
            "roll" => roll(ctx, command).await?,
            _ => return Ok(false),
        }
        Ok(true)
    }
}

async fn roll(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    let sides = command
        .data
        .options
        .get(0)
        .map(|data| {
            data.value
                .as_ref()
                .map(|val| val.as_u64().unwrap())
                .unwrap() as u16
        })
        .unwrap_or(6);

    let num = command
        .data
        .options
        .get(1)
        .map(|data| {
            data.value
                .as_ref()
                .map(|val| val.as_u64().unwrap())
                .unwrap() as u16
        })
        .unwrap_or(1);

    command
        .create_interaction_response(ctx, |res| {
            res.interaction_response_data(|data| {
                data.embed(|embed| {
                    if num == 1 {
                        // Single dice
                        embed.field(
                            format!("{}d{} ", num, sides),
                            rand::random::<u16>() % sides + 1,
                            true,
                        )
                    } else {
                        // Multiple die
                        let mut rolls: Vec<u16> = Vec::new();

                        (0..num).for_each(|_| {
                            rolls.push(rand::random::<u16>() % sides + 1);
                        });

                        let addition = rolls
                            .iter()
                            .map(|roll| roll.to_string())
                            .flat_map(|roll| [roll, " + ".to_string()])
                            .take(rolls.len() * 2 - 1)
                            .collect::<String>();

                        let field = format!("{} = {}", addition, rolls.iter().sum::<u16>());

                        embed.field(format!("{}d{} ", num, sides), field, true)
                    }
                })
            })
        })
        .await?;

    Ok(())
}
