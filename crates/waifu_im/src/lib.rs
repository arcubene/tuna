const API_URL: &str = "https://api.waifu.im/search/";

pub const TAGS: [&str; 8] = [
    "uniform",
    "maid",
    "waifu",
    "marin-kitagawa",
    "mori-calliope",
    "raiden-shogun",
    "oppai",
    "selfies",
];

#[cfg(feature = "nsfw")]
pub const NSFW_TAGS: [&str; 7] = ["ass", "hentai", "milf", "oral", "paizuri", "ecchi", "ero"];

#[derive(serde::Deserialize)]
pub struct Waifu {
    images: Vec<Image>,
}

impl Waifu {
    pub fn images(&self) -> &[Image] {
        self.images.as_ref()
    }
}

#[derive(serde::Deserialize)]
pub struct Image {
    signature: String,

    extension: String,

    image_id: u64,

    favourites: u64,

    dominant_color: String,

    source: String,

    uploaded_at: String,

    is_nsfw: bool,

    width: u64,

    height: u64,

    url: String,

    preview_url: String,

    tags: Vec<Tag>,
}

impl Image {
    pub fn dominant_color(&self) -> &str {
        self.dominant_color.as_ref()
    }

    pub fn extension(&self) -> &str {
        self.extension.as_ref()
    }

    pub fn favourites(&self) -> u64 {
        self.favourites
    }

    pub fn height(&self) -> u64 {
        self.height
    }

    pub fn image_id(&self) -> u64 {
        self.image_id
    }

    pub fn is_nsfw(&self) -> bool {
        self.is_nsfw
    }

    pub fn preview_url(&self) -> &str {
        self.preview_url.as_ref()
    }

    pub fn signature(&self) -> &str {
        self.signature.as_ref()
    }

    pub fn source(&self) -> &str {
        self.source.as_ref()
    }

    pub fn tags(&self) -> &[Tag] {
        self.tags.as_ref()
    }

    pub fn uploaded_at(&self) -> &str {
        self.uploaded_at.as_ref()
    }

    pub fn url(&self) -> &str {
        self.url.as_ref()
    }

    pub fn width(&self) -> u64 {
        self.width
    }
}

#[derive(serde::Deserialize)]
pub struct Tag {
    tag_id: u64,

    name: String,

    description: String,

    is_nsfw: bool,
}

impl Tag {
    pub fn description(&self) -> &str {
        self.description.as_ref()
    }

    pub fn is_nsfw(&self) -> bool {
        self.is_nsfw
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn tag_id(&self) -> u64 {
        self.tag_id
    }
}

pub async fn get_waifu(tag: Option<String>) -> Result<Waifu, reqwest::Error> {
    let mut url = API_URL.to_owned();

    if let Some(category) = tag {
        url = url + "?included_tags=" + &category;
    }

    reqwest::get(url).await?.json().await
}
