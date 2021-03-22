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

async fn get_story(story_id: u32) -> Story {
    reqwest::get(format!("https://hacker-news.firebaseio.com/v0/item/{}.json", story_id))
        .await
        .unwrap()
        .json::<Story>()
        .await
        .unwrap()
}

pub async fn top_stories(max_stories: usize) -> Result<Vec<Story>, Box<dyn std::error::Error>> {
    let mut top_stories: Vec<Story> = Vec::new();

    let resp = reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .await?
        .json::<Vec<u32>>()
        .await?;

    let mut story_futures = Vec::new();
    for (i, story_id) in resp.iter().enumerate() {
        if i == max_stories {
            break;
        }
        let story = get_story(*story_id);
        story_futures.push(story)
    }
    let stories = futures::future::join_all(story_futures).await;

    for story in stories {
        top_stories.push(story);
    }

    Ok(top_stories)
}

async fn get_comment(comment_id: i32) -> Comment {
    reqwest::get(format!("https://hacker-news.firebaseio.com/v0/item/{}.json", comment_id))
        .await
        .unwrap()
        .json::<Comment>()
        .await
        .unwrap()
}

pub async fn get_comments(comment_parents: Vec<i32>) -> Result<Vec<Comment>, Box<dyn std::error::Error>> {
    let mut comments: Vec<Comment> = Vec::new();

    let mut comments_futures = Vec::new();
    for comment_id in comment_parents {
        let comment = get_comment(comment_id);
        comments_futures.push(comment);
    }

    let futures = futures::future::join_all(comments_futures).await;

    for comment in futures {
        comments.push(comment);
    }

    Ok(comments)
}
