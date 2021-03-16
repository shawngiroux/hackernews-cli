#[derive(serde::Deserialize, Debug)]
pub struct Story {
    pub by: String,
    pub descendants: i32,
    pub id: i32,

    #[serde(default = "default_vec")]
    pub kids: Vec<i32>,

    pub score: i32,
    pub time: u32,
    pub title: String,
    pub r#type: String,

    #[serde(default = "default_string")]
    pub url: String
}

fn default_vec() -> Vec<i32> {
    Vec::new()
}

fn default_string() -> String {
    String::new()
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
