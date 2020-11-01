use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod command;
pub mod bookshelf;

pub use crate::command::*;
pub use crate::bookshelf::*;

pub type Result = std::result::Result<(), AlfError>;

#[derive(Error, Debug)]
pub enum AlfError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    EnvError(#[from] std::env::VarError),

    #[error("Argument was not found")]
    HashMapError,

    #[error("Bookmark was not found")]
    BookmarkNotFound,

    #[error("No subcommand specified")]
    NoSubcommand,

    #[error("Invalid subcommand specified")]
    InvalidSubcommand,

    #[error("Unknown error is occured")]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Config {
    pub bookmark_file: String
}

impl Config {
    pub fn from_file(file: &str) -> std::result::Result<Config, std::io::Error> {
        use std::fs::File;
        use std::io::prelude::*;
        let mut file = File::open(file)?;

        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let config = toml::from_str(content.as_str())?;
        Ok(config)
    }
}
