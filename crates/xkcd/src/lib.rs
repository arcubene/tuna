const API_URL: &str = "https://xkcd.com/{}/info.0.json";
const API_LATEST_URL: &str = "https://xkcd.com/info.0.json";

#[derive(serde::Deserialize)]
pub struct Xkcd {
    month: String,

    num: u32,

    link: String,

    year: String,

    news: String,

    safe_title: String,

    transcript: String,

    alt: String,

    img: String,

    title: String,

    day: String,
}

impl Xkcd {
    pub fn alt(&self) -> &str {
        self.alt.as_ref()
    }

    pub fn day(&self) -> &str {
        self.day.as_ref()
    }

    pub fn img(&self) -> &str {
        self.img.as_ref()
    }

    pub fn link(&self) -> &str {
        self.link.as_ref()
    }

    pub fn month(&self) -> &str {
        self.month.as_ref()
    }

    pub fn news(&self) -> &str {
        self.news.as_ref()
    }

    pub fn num(&self) -> u32 {
        self.num
    }

    pub fn safe_title(&self) -> &str {
        self.safe_title.as_ref()
    }

    pub fn title(&self) -> &str {
        self.title.as_ref()
    }

    pub fn transcript(&self) -> &str {
        self.transcript.as_ref()
    }

    pub fn year(&self) -> &str {
        self.year.as_ref()
    }
}

/// Get a selected xkcd comic.
///
/// # Errors
///
/// This function will return an error if:
/// - There was an error while sending request
/// - The response could not be parsed into Json
pub async fn get_xkcd(num: u32) -> Result<Xkcd, reqwest::Error> {
    let num = num.min(get_xkcd_latest().await?.num).max(1).to_string();
    reqwest::get(API_URL.replace("{}", &num))
        .await?
        .json()
        .await
}

/// Get a random xkcd comic.
///
/// # Errors
///
/// This function will return an error if:
/// - There was an error while sending request
/// - The response could not be parsed into Json
pub async fn get_xkcd_random() -> Result<Xkcd, reqwest::Error> {
    let latest = get_xkcd_latest().await?.num;
    let num = (rand::Rng::gen_range(&mut rand::thread_rng(), 1..=latest)).to_string();

    reqwest::get(API_URL.replace("{}", &num))
        .await?
        .json()
        .await
}

/// Get the latest xkcd comic.
///
/// # Errors
///
/// This function will return an error if:
/// - There was an error while sending request
/// - The response could not be parsed into Json
pub async fn get_xkcd_latest() -> Result<Xkcd, reqwest::Error> {
    let url = API_LATEST_URL.to_owned();
    reqwest::get(url).await?.json().await
}
