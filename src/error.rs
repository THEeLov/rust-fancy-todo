use std::io;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Invalid argument for command {0}")]
    InvalidArgument(String),
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
    #[error("todo not found: {0}")]
    TodoNotFound(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
