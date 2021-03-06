use crossclip::{Clipboard, SystemClipboard};

pub mod event;

use tui::widgets::ListState;
use webbrowser;
use crate::hackernews::stories::Story;
use crate::hackernews::comments::Comment;


#[derive(Debug)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        let mut list = StatefulList {
            state: ListState::default(),
            items,
        };
        list.state.select(Some(0));
        list
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn go_to_top(&mut self) {
        self.state.select(Some(0));
    }

    pub fn go_to_bottom(&mut self) {
        self.state.select(Some(self.items.len() - 1));
    }
}

impl StatefulList<Story> {
    pub fn select(&mut self) {
        let i = match self.state.selected() {
            Some(i) => i,
            None => 0
        };

        let selected_items = &self.items[i];
        webbrowser::open(&selected_items.url).unwrap();
    }

    pub fn get_comments(&mut self) -> Vec<i32>{
        let i = match self.state.selected() {
            Some(i) => i,
            None => 0
        };

        self.items[i].kids.to_vec()
    }
}

impl StatefulList<Comment> {
    pub fn next_parent(&mut self) {
        let i = match self.state.selected() {
            Some(i) => i,
            None => 0,
        };

        for (index, comment) in self.items.iter().enumerate() {
            if index > i && comment.depth == 0 {
                self.state.select(Some(index));
                break;
            }
        }
    }

    pub fn previous_parent(&mut self) {
        let i = match self.state.selected() {
            Some(i) => i,
            None => 0,
        };

        for (index, comment) in self.items.iter().rev().enumerate() {
            // Creating inverse mimic original index with reversed array
            let inverse_index: i32 = (index as i32) - (self.items.len() - 1) as i32;
            let inverse_index = inverse_index.abs() as usize;

            if inverse_index < i && comment.depth == 0 {
                self.state.select(Some(inverse_index));
                break;
            }
        }
    }

    pub fn copy_text_to_clipboard(&mut self) {
        let i = match self.state.selected() {
            Some(i) => i,
            None => 0
        };

        let comment = &self.items[i];

        let clipboard = SystemClipboard::new().unwrap();
        clipboard.set_string_contents(String::from(&comment.text)).unwrap();
    }
}
