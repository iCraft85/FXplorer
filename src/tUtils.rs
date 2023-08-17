use ratatui::{prelude::*, widgets::*};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::path::{Path, PathBuf};

use crate::DirItem;

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}


impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
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

    pub fn getSelected(&mut self) -> Option<usize> {
        return self.state.selected();
    }
}

pub struct DirList {
    pub parent: PathBuf,
    pub items: StatefulList<DirItem>,
    // events: Vec<(&'a str, &'a str)>,
}

impl DirList {
    pub fn new(parent: PathBuf, items: Vec<DirItem>) -> DirList {
        DirList {
            parent: parent,
            items: StatefulList::with_items(items)
        }
    }
}

pub enum Show {
    All,
    FilesAndDirs,
    FilesOnly,
    DirsOnly
}

pub enum SortOrder {
    AToZ,
    ZToA
}

pub struct Config {
    pub show: Show,
    pub sortOrder: SortOrder,
}

impl Config {
    pub fn showNext(&mut self) {
        self.show = match self.show {
            Show::All => Show::FilesAndDirs,
            Show::FilesAndDirs => Show::FilesOnly,
            Show::FilesOnly => Show::DirsOnly,
            Show::DirsOnly => Show::All,
        };
    }

    pub fn showPrev(&mut self) {
        self.show = match self.show {
            Show::All => Show::DirsOnly,
            Show::DirsOnly => Show::FilesOnly,
            Show::FilesOnly => Show::FilesAndDirs,
            Show::FilesAndDirs => Show::All,
        };
    }
}
