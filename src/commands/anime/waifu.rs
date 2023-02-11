use rand::seq::SliceRandom;
use serenity::{
    builder::CreateApplicationCommandOption,
    model::prelude::{
        interaction::{
            application_command::ApplicationCommandInteraction,
            autocomplete::AutocompleteInteraction,
        },
        *,
    },
    prelude::*,
};

pub(super) fn register(
    option: &mut CreateApplicationCommandOption,
) -> &mut CreateApplicationCommandOption {
    option
        .name("waifu")
        .description("Return a waifu image or gif.")
        .kind(command::CommandOptionType::SubCommand)
        .create_sub_option(|sub_option| {
            sub_option
                .name("tag")
                .description("Tag to search.")
                .kind(command::CommandOptionType::String)
                .set_autocomplete(true)
        })
}

pub trait Waifu: Send + Sync {
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

impl Waifu for waifu_im::Waifu {
    fn dominant_color(&self) -> &str {
        self.images()[0]
            .dominant_color()
            .strip_prefix('#')
            .expect("Color")
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

impl Waifu for waifu_pics::Waifu {
    fn url(&self) -> &str {
        self.url()
    }
}

async fn waifu_im(
    tag: Option<String>,
) -> Result<Box<dyn Waifu>, Box<dyn std::error::Error + Send>> {
    waifu_im::get_waifu(tag)
        .await
        .map(|val| Box::new(val) as Box<dyn Waifu>)
        .map_err(|val| Box::new(val) as Box<dyn std::error::Error + std::marker::Send>)
}

async fn waifu_pics(
    tag: Option<String>,
) -> Result<Box<dyn Waifu>, Box<dyn std::error::Error + Send>> {
    waifu_pics::get_waifu(tag)
        .await
        .map(|val| Box::new(val) as Box<dyn Waifu>)
        .map_err(|val| Box::new(val) as Box<dyn std::error::Error + std::marker::Send>)
}

async fn waifu_random(
    tag: Option<String>,
) -> Result<Box<dyn Waifu>, Box<dyn std::error::Error + Send>> {
    if rand::random() {
        waifu_im(tag).await
    } else {
        waifu_pics(tag).await
    }
}

pub(super) async fn waifu(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    let options = interaction
        .data
        .options
        .get(0)
        .expect("Option")
        .options
        .as_slice();

    let tag = options.get(0).and_then(|data| {
        data.value
            .as_ref()
            .and_then(|val| val.as_str())
            .map(ToString::to_string)
    });

    let waifu = match tag {
        Some(tag) => match (
            waifu_im::TAGS.contains(&tag.as_ref()),
            waifu_pics::TAGS.contains(&tag.as_ref()),
        ) {
            (true, true) => waifu_random(Some(tag)).await,
            (true, false) => waifu_im(Some(tag)).await,
            (false, true) => waifu_pics(Some(tag)).await,
            (false, false) => waifu_random(None).await,
        },

        None => waifu_random(None).await,
    }
    .expect("Waifu");

    interaction
        .create_interaction_response(ctx, |res| {
            res.interaction_response_data(|data| {
                data.embed(|embed| {
                    let tags: String = waifu
                        .tags()
                        .iter()
                        .flat_map(|x| [x, ", "])
                        .take(waifu.tags().len() * 2 - 1)
                        .collect();

                    embed
                        .title(format!("{} | <{}>", tags, waifu.source()))
                        .image(waifu.url())
                        .color(i32::from_str_radix(waifu.dominant_color(), 16).expect("Color"))
                })
            })
        })
        .await
}

pub(super) async fn waifu_autocomplete(
    ctx: &Context,
    autocomplete: &AutocompleteInteraction,
) -> Result<(), serenity::Error> {
    let options = autocomplete
        .data
        .options
        .get(0)
        .expect("Option")
        .options
        .as_slice();

    let search_tag = options
        .get(0)
        .expect("Tag Option")
        .value
        .as_ref()
        .expect("Tag Value")
        .as_str()
        .expect("Tag")
        .to_lowercase();

    let mut complete: Vec<&str> = waifu_im::TAGS
        .into_iter()
        .chain(waifu_pics::TAGS.into_iter())
        .filter(|tag| tag.contains(search_tag.as_str()))
        .collect();

    // Remove duplicates
    complete.sort();
    complete.dedup();

    // Discord can only show 25 max, so shuffle the tags every time for variety.
    complete.shuffle(&mut rand::thread_rng());

    autocomplete
        .create_autocomplete_response(ctx, |res| {
            // Discord's max number of autocompletion is 25
            complete.into_iter().take(25).for_each(|val| {
                res.add_string_choice(val, val);
            });
            res
        })
        .await
}
