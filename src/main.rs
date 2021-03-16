#[allow(dead_code)]
mod util;
mod hackernews;

use crate::util::{
    event::{Event, Events},
    StatefulList,
};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    // Create Terminal
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Get initial front page links
    let top_stories = match hackernews::top_stories(10 as usize).await {
        Ok(x) => x,
        Err(error) => panic!("{}", error)
    };

    let mut stateful_list = StatefulList::with_items(top_stories);

    let events = Events::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();

            // Preparing a vector of list items
            let items: Vec<ListItem> = stateful_list
                .items
                .iter()
                .map(|data| {
                    let title = Spans::from(vec![
                        Span::styled(
                            format!("{}", data.title),
                            Style::default()
                        )
                    ]);
                    let url = Spans::from(vec![
                        Span::styled(
                            format!("Comments: {} | URL: {}", data.descendants, data.url),
                            Style::default()
                        )
                    ]);
                    ListItem::new(vec![
                        title,
                        url,
                        Spans::from("")
                    ])
                }).
            collect();

            // Creating the list for rendering
            let items_list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Top Stories"))
                .highlight_style(
                    Style::default()
                        .fg(Color::LightYellow)
                        //.fg(Color::Black)
                        //.bg(Color::LightYellow)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            // Rendering list data
            f.render_stateful_widget(items_list, size, &mut stateful_list.state);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Char('l') => {
                    stateful_list.unselect();
                }
                Key::Char('j') => {
                    stateful_list.next();
                }
                Key::Char('k') => {
                    stateful_list.previous();
                }
                Key::Char('\n') => {
                    stateful_list.select();
                }
                Key::Esc => {
                    stateful_list.unselect();
                }
                _ => {}
            },
            _ => {}
        }
    }

    Ok(())
}
