const API_URL: &str = "https://animechan.vercel.app/api/";

#[derive(serde::Deserialize)]
pub struct Quote {
    anime: String,
    character: String,
    quote: String,
}

impl Quote {
    /// Returns a reference to the anime of this [`Quote`].
    pub fn anime(&self) -> &str {
        self.anime.as_ref()
    }

    /// Returns a reference to the character of this [`Quote`].
    pub fn character(&self) -> &str {
        self.character.as_ref()
    }

    /// Returns a reference to the quote of this [`Quote`].
    pub fn quote(&self) -> &str {
        self.quote.as_ref()
    }
}

pub async fn get_quote_random() -> Result<Quote, reqwest::Error> {
    reqwest::get(API_URL.to_owned() + "random")
        .await?
        .json::<Quote>()
        .await
}

pub async fn get_quote(anime: &str) -> Result<Quote, reqwest::Error> {
    reqwest::get(API_URL.to_owned() + "quotes/anime?title=" + anime)
        .await?
        .json::<Quote>()
        .await
}

pub async fn get_quote_character(name: &str) -> Result<Quote, reqwest::Error> {
    reqwest::get(API_URL.to_owned() + "quotes/character?name=" + name)
        .await?
        .json::<Quote>()
        .await
}
