mod display;
mod hacker_news;

#[tokio::main]
async fn main() {
    let res = hacker_news::api::get::top_stories().await;

    let top_stories = res.unwrap();

    let mut stories = Vec::new();

    let mut count = 0;
    for id in top_stories {
        if count < 10 {
            let story = hacker_news::api::get::story(id).await;
            stories.push(story.unwrap());
        }
        count += 1;
    }

    display::clear_screen();
    display::draw_stories(stories);
}
