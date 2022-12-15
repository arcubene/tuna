const API_URL: &str = "https://xkcd.com/{}/info.0.json";
const API_LATEST_URL: &str = "https://xkcd.com/info.0.json";

#[derive(serde::Deserialize)]
pub struct XkcdQuery {
    month: String,

    num: u64,

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

impl XkcdQuery {
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

    pub fn num(&self) -> u64 {
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

pub async fn get_xkcd(num: u64) -> Result<XkcdQuery, reqwest::Error> {
    let num = num.min(get_xkcd_latest().await?.num).to_string();
    reqwest::get(API_URL.replace("{}", &num))
        .await?
        .json::<XkcdQuery>()
        .await
}

pub async fn get_xkcd_random() -> Result<XkcdQuery, reqwest::Error> {
    let num = (rand::random::<u64>() % get_xkcd_latest().await?.num).to_string();
    reqwest::get(API_URL.replace("{}", &num))
        .await?
        .json::<XkcdQuery>()
        .await
}

pub async fn get_xkcd_latest() -> Result<XkcdQuery, reqwest::Error> {
    let url = API_LATEST_URL.to_owned();
    reqwest::get(url).await?.json::<XkcdQuery>().await
}
