use colored::*;

pub fn clear_screen() {
    print!("\x1B[2J");
}

pub fn draw_stories(stories: Vec<crate::hacker_news::Story>) {
    for story in stories {
        let title = story.title;
        let url = match story.url {
            Some(x) => x,
            None => "No URL Provided".to_string(),
        };

        println!("{}\n{:?}\n\n", title.yellow().bold(), url);
    }
}
