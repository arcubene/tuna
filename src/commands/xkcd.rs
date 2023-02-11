use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{interaction::application_command::ApplicationCommandInteraction, *},
    prelude::*,
};

pub(crate) fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("xkcd")
        .description("Return a xkcd comic.")
        .kind(command::CommandType::ChatInput)
        .create_option(|option| {
            option
                .name("num")
                .description("xkcd to get.")
                .kind(command::CommandOptionType::Integer)
                .min_int_value(0)
        })
}

pub(crate) async fn xkcd(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    let num = interaction.data.options.get(0).and_then(|data| {
        data.value
            .as_ref()
            .and_then(|val| val.as_u64().map(|val| val as u32))
    });

    let xkcd = match num {
        Some(val) => xkcd::get_xkcd(val).await,
        None => xkcd::get_xkcd_random().await,
    }
    .expect("xkcd");

    let title = format!("{} | {} | {}", xkcd.title(), xkcd.link(), xkcd.num());
    let date = format!("{}/{}/{}", xkcd.year(), xkcd.month(), xkcd.day());

    interaction
        .create_interaction_response(ctx, |res| {
            res.interaction_response_data(|data| {
                data.embed(|embed| {
                    embed
                        .description(title)
                        .image(xkcd.img())
                        .footer(|footer| footer.text(date))
                })
            })
        })
        .await
}
