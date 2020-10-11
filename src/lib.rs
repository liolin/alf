use thiserror::Error;

pub mod command;
pub mod bookshelf;

pub use crate::command::*;
pub use crate::bookshelf::*;

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


pub type Result = std::result::Result<(), AlfError>;
