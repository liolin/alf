use std::env;

use crate::bookshelf::Bookshelf;
use crate::{AlfError, Result};

pub trait Command {
    fn run(&self) -> Result;
}

pub struct List<'a> {
    // args: HashMap<String, String>,
    args: &'a clap::ArgMatches<'a>

}

pub struct Open<'a> {
    // args: HashMap<String, String>
        args: &'a clap::ArgMatches<'a>
}

impl<'a> Command for List<'a> {
    fn run(&self) -> Result {
        let home = env::var("HOME")?;
        let bookshelf = Bookshelf::from_file(format!("{}/.alf.toml", home).as_str())?;

        if let Some(tag) = self.args.value_of("tag") {
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

impl<'a> List<'a> {
    pub fn with_arguments(args: &'a clap::ArgMatches<'a>) -> Self {
        Self {
            args
        }
    }
}

impl<'a> Command for Open<'a> {
    fn run(&self) -> Result {
        let home = env::var("HOME")?;
        let bookshelf = Bookshelf::from_file(format!("{}/.alf.toml", home).as_str())?;

        let x =  bookshelf
            .find_by_name(self.args.value_of("name")
                          .ok_or(AlfError::HashMapError)?);

        match x {
            Some(bookmark) => {
                let _ = webbrowser::open(bookmark.url.as_str())?;
                Ok(())
            },
            None => Err(AlfError::BookmarkNotFound)
        }
    }
}

impl<'a> Open<'a> {
    pub fn with_arguments(args: &'a clap::ArgMatches<'a>) -> Self {
        Self {
            args
        }
    }
}
