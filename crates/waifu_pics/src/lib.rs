const API_URL: &str = "https://api.waifu.pics/";

pub const TAGS: [&str; 31] = [
    "waifu", "neko", "shinobu", "megumin", "bully", "cuddle", "cry", "hug", "awoo", "kiss", "lick",
    "pat", "smug", "bonk", "yeet", "blush", "smile", "wave", "highfive", "handhold", "nom", "bite",
    "glomp", "slap", "kill", "kick", "happy", "wink", "poke", "dance", "cringe",
];

pub async fn get_waifu(tag: Option<String>) -> Result<WaifuPicQuery, reqwest::Error> {
    let mut url = API_URL.to_owned() + "sfw/";
    match tag {
        Some(category) => url += &category,
        None => url += "waifu",
    }
    reqwest::get(url).await?.json::<WaifuPicQuery>().await
}

#[cfg(feature = "nsfw")]
pub async fn get_waifu_nsfw(tag: Option<String>) -> Result<WaifuPicQuery, reqwest::Error> {
    let mut url = API_URL.to_owned() + "nsfw/";
    match tag {
        Some(category) => url += &category,
        None => url += "waifu",
    }
    reqwest::get(url).await?.json::<WaifuPicQuery>().await
}

#[derive(serde::Deserialize)]
pub struct WaifuPicQuery {
    url: String,
}

impl WaifuPicQuery {
    pub fn url(&self) -> &str {
        self.url.as_ref()
    }
}
