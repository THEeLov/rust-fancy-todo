use std::io;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Reading line from stdin failed")]
    ReadLine(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
