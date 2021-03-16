pub mod event;

use tui::widgets::ListState;
use webbrowser;
use crate::hackernews::Story;


#[derive(Debug)]
pub struct StatefulList {
    pub state: ListState,
    pub items: Vec<Story>,
}

impl StatefulList {
    pub fn new() -> StatefulList {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<Story>) -> StatefulList {
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

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn select(&mut self) {
        let i = match self.state.selected() {
            Some(i) => i,
            None => 0
        };

        let selected_items = &self.items[i];
        webbrowser::open(&selected_items.url).unwrap();
    }
}
