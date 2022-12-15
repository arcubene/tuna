use serenity::{
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

pub(super) async fn quote(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    let quote_query = animechan::get_quote_random().await;

    command
        .create_interaction_response(ctx, |res| {
            res.interaction_response_data(|data| match quote_query {
                Ok(quote) => data.embed(|embed| {
                    embed
                        .description(format!("\"{}\"", quote.quote()))
                        .footer(|footer| {
                            footer.text(format!("- {} ({})", quote.character(), quote.anime()))
                        })
                }),
                Err(why) => data.content(format!("Error: {:?}", why)),
            })
        })
        .await?;

    Ok(())
}
