use std::io;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Reading line from stdin failed")]
    ReadLine(#[from] io::Error),
    #[error("Invalid argument for command {0}")]
    InvalidArgument(String),
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
