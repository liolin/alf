use std::env;
use std::collections::HashMap;

use crate::bookshelf::Bookshelf;
use crate::{AlfError, Result};

pub trait Command {
    fn run(&self) -> Result;
}

pub struct List {
    args: HashMap<String, String>
}

pub struct Open {
    args: HashMap<String, String>
}

impl Command for List {
    fn run(&self) -> Result {
        let home = env::var("HOME")?;
        let bookshelf = Bookshelf::from_file(format!("{}/.alf.toml", home).as_str())?;

        if let Some(tag) = self.args.get("tag") {
            for bookmark in bookshelf.find_by_tag(tag) {
                println!("{}:\n\t{}", bookmark.name, bookmark.url);
            }
        } else {
            for bookmark in bookshelf.bookmarks {
                println!("{}:\n\t{}", bookmark.name, bookmark.url);
                if bookmark.tags.len() > 0 {
                    println!("\tTags: {}", bookmark.tags.join(" "));
                }
            }
        }
        Ok(())
    }
}

impl List {
    pub fn with_arguments(args: HashMap<String, String>) -> Self {
        Self {
            args
        }
    }
}

impl Command for Open {
    fn run(&self) -> Result {
        let home = env::var("HOME")?;
        let bookshelf = Bookshelf::from_file(format!("{}/.alf.toml", home).as_str())?;

        let x =  bookshelf
            .find_by_name(self.args.get("name")
                          .ok_or(AlfError::HashMapError)?
                          .as_str());

        match x {
            Some(bookmark) => {
                let _ = webbrowser::open(bookmark.url.as_str())?;
                Ok(())
            },
            None => Err(AlfError::BookmarkNotFound)
        }
    }
}

impl Open {
    pub fn with_arguments(args: HashMap<String, String>) -> Self {
        Self {
            args
        }
    }
}
