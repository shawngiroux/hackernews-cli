#[derive(serde::Deserialize, Debug)]
pub struct Story {
    pub by: String,
    pub descendants: u32,
    pub id: u32,
    pub kids: Option<Vec<u32>>,
    pub score: u32,
    pub time: u32,
    pub title: String,
    pub url: Option<String>,
}

pub mod api {
    pub mod get {
        pub async fn top_stories() -> Result<Vec<u32>, Box<dyn std::error::Error>> {
            let res = reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json")
                .await?
                .json::<Vec<u32>>()
                .await?;
            Ok(res)
        }
        pub async fn story(
            id: u32,
        ) -> Result<crate::hacker_news::Story, Box<dyn std::error::Error>> {
            let url_string = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
            let url = reqwest::Url::parse(&url_string)?;
            let res = reqwest::get(url)
                .await?
                .json::<crate::hacker_news::Story>()
                .await?;
            Ok(res)
        }
    }
}
