use crate::bookshelf::Bookshelf;
use crate::{AlfError, Result};
use crate::Config;

pub trait Command {
    fn run(&self, config: &Config) -> Result;
}

pub struct List<'a> {
    args: &'a clap::ArgMatches<'a>

}

pub struct Open<'a> {
    args: &'a clap::ArgMatches<'a>
}

impl<'a> Command for List<'a> {
    fn run(&self, config: &Config) -> Result {
        let bookshelf = Bookshelf::from_file(config.bookmark_file.as_str())?;

        match self.args.value_of("tag") {
            Some(tag) => {
                for bookmark in bookshelf.find_by_tag(tag) {
                    bookmark.write(&mut std::io::stdout())?;
                }
            }
            None => {
                bookshelf.write(&mut std::io::stdout())?;
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
    fn run(&self, config: &Config) -> Result {
        let bookshelf = Bookshelf::from_file(config.bookmark_file.as_str())?;

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
