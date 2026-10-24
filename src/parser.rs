use crate::error::{self, AppError};
use std::str::FromStr;

pub enum Command {
    Add { name: String },
    Remove { name: String },
    Done { name: String },
    List,
    Exit,
}

impl FromStr for Command {
    type Err = AppError;

    fn from_str(input: &str) -> error::Result<Self> {
        let (command, rest) = input.split_once(' ').unwrap_or((input, ""));

        let arg = || {
            if rest.is_empty() {
                Err(AppError::InvalidArgument(rest.to_string()))
            } else {
                Ok(rest.to_string())
            }
        };

        match command.to_lowercase().as_str() {
            "exit" => Ok(Command::Exit),
            "list" => Ok(Command::List),
            "add" => Ok(Command::Add { name: arg()? }),
            "remove" => Ok(Command::Remove { name: arg()? }),
            "done" => Ok(Command::Done { name: arg()? }),
            _ => Err(AppError::InvalidCommand(String::from(command))),
        }
    }
}
