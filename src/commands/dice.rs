use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{interaction::application_command::ApplicationCommandInteraction, *},
    prelude::*,
};

pub(crate) fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("roll")
        .description("Roll dice.")
        .kind(command::CommandType::ChatInput)
        .create_option(|option| {
            option
                .name("sides")
                .description("Sides of the dice.")
                .kind(command::CommandOptionType::Integer)
                .min_int_value(1)
                .max_int_value(120)
        })
        .create_option(|option| {
            option
                .name("num")
                .description("Number of dice.")
                .kind(command::CommandOptionType::Integer)
                .min_int_value(1)
                .max_int_value(120)
        })
}

pub(crate) async fn roll(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    let options = interaction.data.options.as_slice();

    let sides = options
        .get(0)
        .and_then(|data| {
            data.value
                .as_ref()
                .and_then(|val| val.as_u64().map(|i| i as u8))
        })
        .unwrap_or(6);

    let quantity = options
        .get(1)
        .and_then(|data| {
            data.value
                .as_ref()
                .and_then(|val| val.as_u64().map(|i| i as u8))
        })
        .unwrap_or(1);

    let field = if quantity == 1 {
        // Single die.
        die::roll(sides).to_string()
    } else {
        // Multiple dice.
        let mut rolls: Vec<u8> = Vec::with_capacity(usize::from(quantity));

        (0..quantity).for_each(|_| {
            rolls.push(die::roll(sides));
        });

        let equation: String = rolls
            .iter()
            .map(ToString::to_string)
            .flat_map(|roll| [roll, " + ".to_string()])
            .take(usize::from(quantity) * 2 - 1)
            .collect();

        format!(
            "{} = {}",
            equation,
            rolls.into_iter().map(u32::from).sum::<u32>()
        )
    };

    interaction
        .create_interaction_response(ctx, |res| {
            res.interaction_response_data(|data| {
                data.embed(|embed| embed.field(format!("{}d{}", quantity, sides), field, true))
            })
        })
        .await
}
