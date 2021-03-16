#[derive(serde::Deserialize, Debug)]
pub struct Story {
    #[serde(default)]
    pub by: String,

    #[serde(default)]
    pub descendants: i32,

    #[serde(default)]
    pub id: i32,

    #[serde(default)]
    pub kids: Vec<i32>,

    #[serde(default)]
    pub score: i32,

    #[serde(default)]
    pub time: u32,

    #[serde(default)]
    pub title: String,

    #[serde(default)]
    pub r#type: String,

    #[serde(default)]
    pub url: String
}

pub async fn top_stories(max_stories: usize) -> Result<Vec<Story>, Box<dyn std::error::Error>> {
    let mut top_stories: Vec<Story> = Vec::new();

    let resp = reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .await?
        .json::<Vec<u32>>()
        .await?;

    for (i, story_id) in resp.iter().enumerate() {
        if i == max_stories {
            break;
        }

        let story = reqwest::get(format!("https://hacker-news.firebaseio.com/v0/item/{}.json", *story_id))
            .await?
            .json::<Story>()
            .await?;

        top_stories.push(story)
    }

    Ok(top_stories)
}
