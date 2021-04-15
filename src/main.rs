#[allow(dead_code)]
mod util;
mod logging;
mod hackernews;

use crate::{
    util::{
        event::{Event, Events},
        StatefulList
    },
    hackernews::{
        stories::Story,
        comments::Comment
    }
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

enum AppState {
    Stories,
    Comments
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    // Create Terminal
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    crate::logging::init_logging();

    // Get initial front page links
    let top_stories = match hackernews::stories::top_stories(25 as usize).await {
        Ok(x) => x,
        Err(error) => panic!("{}", error)
    };

    let mut stateful_list = StatefulList::<Story>::with_items(top_stories);
    let mut comment_list = StatefulList::<Comment>::new();

    let mut events = Events::new();
    events.disable_exit_key();

    let mut current_state = AppState::Stories;

    loop {
        terminal.draw(|f| {
            let size = f.size();

            match current_state {
                AppState::Stories => {
                    // Preparing a vector of list items
                    let items: Vec<ListItem> = stateful_list
                        .items
                        .iter()
                        .enumerate()
                        .map(|(i, data)| {
                            let title = Spans::from(vec![
                                Span::styled(
                                    format!("{}) {}", i + 1, data.title),
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
                        .block(
                            Block::default()
                            .borders(Borders::ALL)
                            .title(
                                Span::styled(" Top Stories ",
                                    Style::default()
                                    .add_modifier(Modifier::BOLD),
                                )
                            )
                        )
                        .highlight_style(
                            Style::default()
                            .fg(Color::LightYellow)
                            .add_modifier(Modifier::BOLD),
                        )
                        .highlight_symbol(">> ");

                    // Rendering list data
                    f.render_stateful_widget(items_list, size, &mut stateful_list.state);
                }

                AppState::Comments => {

                    // Preparing a vector of list items
                    let items: Vec<ListItem> = comment_list
                        .items
                        .iter()
                        .map(|comment| {
                            let mut list_item: Vec<Spans> = vec![];

                            // Width of terminal
                            let width = size.width as usize;

                            let depth_buffer = "  ".repeat(comment.depth as usize);

                            // Wrapping minus a width of 7 to hopefully offset the
                            // highlight characters and borders and pipe delimiter
                            //
                            // It's not ideal to continue adding to this math, and a
                            // better way should be written into the inevitable refactor
                            // of generating these interfaces.
                            let text = textwrap::fill(&comment.text, (width - 7) - (comment.depth * 2) as usize);

                            // Pushing the string splits into the display vector
                            for s in text.split('\n') {
                                let item = Spans::from(vec![
                                    Span::styled(
                                        format!("{}| {}", depth_buffer, s),
                                        Style::default()
                                    )
                                ]);
                                list_item.push(item);
                            }

                            // Name of user who published a comment
                            let user = Spans::from(vec![
                                Span::styled(
                                    format!("{}| By: {}", depth_buffer, comment.by),
                                    Style::default()
                                )
                            ]);

                            // Final pushes for display vector
                            list_item.push(user);
                            list_item.push(Spans::from(""));

                            ListItem::new(list_item)
                        })
                    .collect();

                    // Creating the list for rendering
                    let items_list = List::new(items)
                        .block(
                            Block::default()
                            .borders(Borders::ALL)
                            .title(
                                Span::styled(" Comments ",
                                    Style::default()
                                    .add_modifier(Modifier::BOLD),
                                )
                            )
                        )
                        .highlight_style(
                            Style::default()
                            .fg(Color::LightYellow)
                            .add_modifier(Modifier::BOLD),
                        )
                        .highlight_symbol(">> ");

                    // Rendering list data
                    f.render_stateful_widget(items_list, size, &mut comment_list.state);
                }
            }
        })?;

        match current_state {
            AppState::Stories => {
                match events.next()? {
                    Event::Input(input) => match input {
                        Key::Char('q') => {
                            break;
                        }
                        Key::Char('c') => {
                            // Retrieve comment parents from selected story
                            let comment_parents = stateful_list.get_comments();
                            let comments = match hackernews::comments::get_comments(&comment_parents, 0).await {
                                Ok(x) => x,
                                Err(error) => panic!("{}", error)
                            };
                            let comments = hackernews::comments::flatten_comments(&comments);

                            for comment in comments {
                                comment_list.items.push(comment);
                                comment_list.state.select(Some(0));
                            }
                            current_state = AppState::Comments;
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
                        Key::Char('g') => {
                            stateful_list.go_to_top();
                        }
                        Key::Char('G') => {
                            stateful_list.go_to_bottom();
                        }
                        Key::Char('r') => {
                            let top_stories = match hackernews::stories::top_stories(25 as usize).await {
                                Ok(x) => x,
                                Err(error) => panic!("{}", error)
                            };
                            stateful_list = StatefulList::with_items(top_stories);
                        }
                        _ => {}
                    },

                    Event::Tick => {}
                }
            }

            AppState::Comments => {
                match events.next()? {
                    Event::Input(input) => match input {
                        Key::Char('q') => {
                            comment_list = StatefulList::<Comment>::new();
                            current_state = AppState::Stories;
                        }
                        Key::Char('j') => {
                            comment_list.next();
                        }
                        Key::Char('J') => {
                            comment_list.next_parent();
                        }
                        Key::Char('k') => {
                            comment_list.previous();
                        }
                        Key::Char('K') => {
                            comment_list.previous_parent();
                        }
                        Key::Char('g') => {
                            comment_list.go_to_top();
                        }
                        Key::Char('G') => {
                            comment_list.go_to_bottom();
                        }
                        Key::Char('y') => {
                            comment_list.copy_text_to_clipboard();
                        }
                        _ => {}
                    },

                    Event::Tick => {}
                }
            }
        }
    }

    Ok(())
}
