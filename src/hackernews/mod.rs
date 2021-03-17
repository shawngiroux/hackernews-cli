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

#[derive(serde::Deserialize, Debug)]
pub struct Comment {
    #[serde(default)]
    pub by: String,

    #[serde(default)]
    pub id: i32,

    #[serde(default)]
    pub kids: Vec<i32>,

    #[serde(default)]
    pub parent: i32,

    #[serde(default)]
    pub text: String,

    #[serde(default)]
    pub time: u32,

    #[serde(default)]
    pub r#type: String
}

pub fn top_stories(max_stories: usize) -> Result<Vec<Story>, Box<dyn std::error::Error>> {
    let mut top_stories: Vec<Story> = Vec::new();

    let resp = reqwest::blocking::get("https://hacker-news.firebaseio.com/v0/topstories.json")?
        .json::<Vec<u32>>()
        .unwrap();

    for (i, story_id) in resp.iter().enumerate() {
        if i == max_stories {
            break;
        }

        let story = reqwest::blocking::get(format!("https://hacker-news.firebaseio.com/v0/item/{}.json", *story_id))?
            .json::<Story>()
            .unwrap();

        top_stories.push(story)
    }

    Ok(top_stories)
}

pub fn get_comments(comment_id: i32) -> Result<Comment, Box<dyn std::error::Error>> {
    log::info!("Getting comment for: {}", comment_id);
    let comment = reqwest::blocking::get(format!("https://hacker-news.firebaseio.com/v0/item/{}.json", comment_id))?
        .json::<Comment>()
        .unwrap();
    log::info!("Getting comment for: {}", comment_id);
    Ok(comment)
}
