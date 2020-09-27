use std::env;
use std::collections::HashMap;

use crate::Bookshelf;

pub trait Command {
    fn run(self);
    fn with_arguments(args: HashMap<String, String>) -> Self;
}

pub struct List {
    args: HashMap<String, String>
}

pub struct Open {
    args: HashMap<String, String>
}

impl Command for List {
    fn run(self) {
        let home = env::var("HOME").unwrap();
        let bookshelf = Bookshelf::from_file(format!("{}/.alf.toml", home).as_str());

        if let Some(tag) = self.args.get("tag") {
            for bookmark in bookshelf.find_by_tag(tag) {
                println!("{}:\n\t{}", bookmark.name, bookmark.url);
            }
        } else {
            for bookmark in bookshelf.bookmarks {
                println!("{}:\n\t{}", bookmark.name, bookmark.url);
            }
        }
    }

    fn with_arguments(args: HashMap<String, String>) -> Self {
        Self {
            args
        }
    }
}

impl Command for Open {
    fn run(self) {
        let home = env::var("HOME").unwrap();
        let bookshelf = Bookshelf::from_file(format!("{}/.alf.toml", home).as_str());
        if let Some(bookmark) = bookshelf.find_by_name(self.args.get("name").unwrap().as_str()) {
            if webbrowser::open(bookmark.url.as_str()).is_ok() {

            }
        }

    }

    fn with_arguments(args: HashMap<String, String>) -> Self {
        Self {
            args
        }
    }
}
