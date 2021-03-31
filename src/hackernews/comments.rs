use regex::Regex;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Comment {
    #[serde(default)]
    pub by: String,

    #[serde(default)]
    pub id: i32,

    #[serde(default)]
    pub kids: Vec<i32>,

    #[serde(default)]
    pub kid_comments: Vec<Comment>,

    #[serde(default)]
    pub depth: i32,

    #[serde(default)]
    pub parent: i32,

    #[serde(default)]
    pub text: String,

    #[serde(default)]
    pub time: u32,

    #[serde(default)]
    pub r#type: String
}

async fn get_comment(comment_id: i32) -> Comment {
    reqwest::get(format!("https://hacker-news.firebaseio.com/v0/item/{}.json", comment_id))
        .await
        .unwrap()
        .json::<Comment>()
        .await
        .unwrap()
}

pub fn flatten_comments(comments: &Vec<Comment>) -> Vec<Comment>{
    let mut flat_comments: Vec<Comment> = Vec::new();

    for comment in comments {
        flat_comments.push(comment.clone());
        if comment.kids.len() > 0 {
            let kid_comments = flatten_comments(&comment.kid_comments);
            for kid_comment in kid_comments {
                flat_comments.push(kid_comment);
            }
        }
    }

    flat_comments
}

#[async_recursion::async_recursion]
pub async fn get_comments(comment_parents: &Vec<i32>, depth: i32) -> Result<Vec<Comment>, Box<dyn std::error::Error>> {
    let mut comments: Vec<Comment> = Vec::new();

    let mut comments_futures = Vec::new();
    for comment_id in comment_parents {
        let comment = get_comment(*comment_id);
        comments_futures.push(comment);
    }

    let mut futures = futures::future::join_all(comments_futures).await;

    for comment in &mut futures {
        // Unsure why, but occasionally we are getting empty users
        if comment.by == "" {
            continue;
        }

        // Cleaning up string to displaying purposes
        comment.text = decode_comments(&comment.text);
        comment.text = remove_html_tags(&comment.text);

        comment.depth = depth;
        if comment.kids.len() > 0 {
            let depth = depth + 1;
            let kid_comments = get_comments(&comment.kids, depth).await;
            comment.kid_comments = match kid_comments {
                Ok(x) => x,
                Err(error) => panic!("{}", error)
            };
        }

        comments.push(comment.clone());
    }

    Ok(comments)
}

fn remove_html_tags(text: &str) -> String {
    // Removing paragraph tags which typically seem to appear immediately after a period
    let re = Regex::new(r"<[p].*?>").unwrap();
    let mut text = re.replace_all(text, " ").to_string();

    // Removing anchor tags an inserting the full url
    let mut urls = Vec::new();
    let mut urls_to_replace = Vec::new();

    // Extracting urls that we will use for replacement
    for mat in Regex::new("\"(http|https).*?\"").unwrap().find_iter(&text) {
        let url = text[mat.start()..mat.end()]
            .to_string()
            .replace("\"", "");
        urls.push(url);
    }

    // Finding any anchor tags that we will replace
    for mat in Regex::new(r"<a.*?/a>").unwrap().find_iter(&text) {
        let replace_val = text[mat.start()..mat.end()]
            .to_string();
        urls_to_replace.push(replace_val);
    }

    // Replacing the anchor tags with the links
    for (i, url) in urls_to_replace.iter().enumerate() {
        text = text.replace(url, &urls[i]);
    }

    text
}

fn decode_comments(text: &str) -> String {
    match htmlescape::decode_html(text) {
        Ok(text) => text,
        Err(error) => panic!("{:?}", error)
    }
}
