use rand::seq::SliceRandom;
use serenity::{
    model::prelude::interaction::{
        application_command::ApplicationCommandInteraction, autocomplete::AutocompleteInteraction,
    },
    prelude::Context,
};

pub(super) async fn waifu(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    let wanted_tag = command
        .data
        .options
        .get(0)
        .unwrap()
        .options
        .get(0)
        .map(|data| {
            data.value
                .as_ref()
                .map(|val| val.as_str().unwrap().to_string())
                .unwrap()
        });

    let waifu = query_waifu(wanted_tag).await;

    command
        .create_interaction_response(ctx, |res| {
            res.interaction_response_data(|data| match waifu {
                Ok(waifu_card) => data.embed(|embed| {
                    let tags: String = waifu_card
                        .tags()
                        .iter()
                        .flat_map(|x| [x, ", "])
                        .take(waifu_card.tags().len() * 2 - 1)
                        .collect();

                    embed
                        .title(format!("{} | <{}>", tags, waifu_card.source()))
                        .image(waifu_card.url())
                        .color(i32::from_str_radix(waifu_card.dominant_color(), 16).unwrap())
                }),
                Err(why) => data.content(format!("Error: {:?}", why)),
            })
        })
        .await?;

    Ok(())
}

pub(super) async fn waifu_autocomplete(
    ctx: &Context,
    autocomplete: &AutocompleteInteraction,
) -> Result<(), serenity::Error> {
    if let Some(val) = autocomplete
        .data
        .options
        .get(0)
        .unwrap()
        .options
        .get(0)
        .unwrap()
        .value
        .as_ref()
    {
        let val = val.as_str().unwrap().to_lowercase();

        let mut complete: Vec<&str> = waifu_im::TAGS
            .into_iter()
            .filter(|tag| tag.contains(val.as_str()))
            .chain(
                waifu_pics::TAGS
                    .into_iter()
                    .filter(|tag| tag.contains(val.as_str())),
            )
            .collect();

        // Remove duplicates
        complete.sort();
        complete.dedup();

        // Discord can only show 25 max, so shuffle the tags every time
        complete.shuffle(&mut rand::thread_rng());

        autocomplete
            .create_autocomplete_response(ctx, |res| {
                // Discord's max number of autocompletion is 25
                for val in complete.into_iter().take(25) {
                    res.add_string_choice(val, val);
                }
                res
            })
            .await?;
    }

    Ok(())
}

pub trait WaifuCard: Send + Sync {
    /// Return the dominant color of the [`Waifu`] image as a hex code.
    fn dominant_color(&self) -> &str {
        "000000"
    }

    /// Return the source of the [`Waifu`] image.
    fn source(&self) -> &str {
        self.url()
    }

    /// Return the tags associated with the [`Waifu`] image.
    fn tags(&self) -> Vec<&str> {
        vec!["waifu"]
    }

    /// Return the url of the [`Waifu`] image.
    fn url(&self) -> &str;
}

impl WaifuCard for waifu_im::WaifuImQuery {
    fn dominant_color(&self) -> &str {
        self.images()[0].dominant_color().strip_prefix('#').unwrap()
    }

    fn source(&self) -> &str {
        self.images()[0].source()
    }

    fn tags(&self) -> Vec<&str> {
        self.images()[0]
            .tags()
            .iter()
            .map(|tag| tag.name())
            .collect()
    }

    fn url(&self) -> &str {
        self.images()[0].url()
    }
}

impl WaifuCard for waifu_pics::WaifuPicQuery {
    fn url(&self) -> &str {
        self.url()
    }
}

async fn query_waifu(
    tag: Option<String>,
) -> Result<Box<dyn WaifuCard>, Box<dyn std::error::Error + Send>> {
    if let Some(tag) = tag {
        match (
            waifu_im::TAGS.contains(&tag.as_ref()),
            waifu_pics::TAGS.contains(&tag.as_ref()),
        ) {
            (true, true) => {
                if rand::random() {
                    query_waifu_im(Some(tag)).await
                } else {
                    query_waifu_pics(Some(tag)).await
                }
            }
            (true, false) => query_waifu_im(Some(tag)).await,
            (false, true) => query_waifu_pics(Some(tag)).await,
            (false, false) => {
                if rand::random() {
                    query_waifu_im(None).await
                } else {
                    query_waifu_pics(None).await
                }
            }
        }
    } else if rand::random() {
        query_waifu_im(None).await
    } else {
        query_waifu_pics(None).await
    }
}

async fn query_waifu_im(
    tag: Option<String>,
) -> Result<Box<dyn WaifuCard>, Box<dyn std::error::Error + Send>> {
    waifu_im::get_waifu(tag)
        .await
        .map(|val| Box::new(val) as Box<dyn WaifuCard>)
        .map_err(|val| Box::new(val) as Box<dyn std::error::Error + std::marker::Send>)
}

async fn query_waifu_pics(
    tag: Option<String>,
) -> Result<Box<dyn WaifuCard>, Box<dyn std::error::Error + Send>> {
    waifu_pics::get_waifu(tag)
        .await
        .map(|val| Box::new(val) as Box<dyn WaifuCard>)
        .map_err(|val| Box::new(val) as Box<dyn std::error::Error + std::marker::Send>)
}
