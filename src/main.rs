#[allow(dead_code)]
mod util;
mod logging;
mod hackernews;

use futures::executor::block_on;
use crate::util::{
    event::{Event, Events},
    StatefulList,
    CommentStatefulList
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

fn main() -> Result<(), Box<dyn Error>>{
    // Create Terminal
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    crate::logging::init_logging();

    // Get initial front page links
    let top_stories = match hackernews::top_stories(15 as usize) {
        Ok(x) => x,
        Err(error) => panic!("{}", error)
    };

    let mut stateful_list = StatefulList::with_items(top_stories);
    let mut comment_list = CommentStatefulList::new();

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
                            let msg = Spans::from(vec![
                                Span::styled(
                                    format!("{}", comment.text),
                                    Style::default()
                                )
                            ]);
                            let user = Spans::from(vec![
                                Span::styled(
                                    format!("By: {}", comment.by),
                                    Style::default()
                                )
                            ]);
                            ListItem::new(vec![
                                msg,
                                user,
                                Spans::from("")
                            ])
                        }).
                    collect();

                    // Creating the list for rendering
                    let items_list = List::new(items)
                        .block(Block::default().borders(Borders::ALL).title("Comments"))
                        .highlight_style(
                            Style::default()
                            .fg(Color::LightYellow)
                            .add_modifier(Modifier::BOLD),
                        )
                        .highlight_symbol(">> ");

                    // Rendering list data
                    f.render_stateful_widget(items_list, size, &mut comment_list.state);
                    let comments = Block::default()
                        .title("Comments")
                        .borders(Borders::ALL);
                    f.render_widget(comments, size);
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

                            for comment_id in comment_parents {
                                let comment = hackernews::get_comments(comment_id);
                                let comment = match comment {
                                    Ok(x) => x,
                                    Err(error) => panic!("{}", error)
                                };

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
                        _ => {}
                    },

                    Event::Tick => {}
                }
            }

            AppState::Comments => {
                match events.next()? {
                    Event::Input(input) => match input {
                        Key::Char('q') => {
                            comment_list = CommentStatefulList::new();
                            current_state = AppState::Stories;
                        }
                        Key::Char('j') => {
                            comment_list.next();
                        }
                        Key::Char('k') => {
                            comment_list.previous();
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
