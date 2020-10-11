use thiserror::Error;

pub mod command;
pub mod bookshelf;

pub use crate::command::*;
pub use crate::bookshelf::*;

#[derive(Error, Debug)]
pub enum AlfError {
    #[error(transparent)]
    Webbrowser(#[from] std::io::Error),
    #[error(transparent)]
    EnvError(#[from] std::env::VarError),
    #[error("Argument was not found")]
    HashMapError,
    #[error("Bookmark was not found")]
    BookmarkNotFound,
    #[error("unknown data store error")]
    Unknown,
}


pub type Result = std::result::Result<(), AlfError>;
