use std::str::FromStr;

use crate::error::{self, AppError};

enum Command {
    Add { name: String },
    Remove { id: String },
    Done { id: String },
    List,
    Exit,
}

impl FromStr for Command {
    type Err = AppError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
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
            "remove" => Ok(Command::Remove { id: arg()? }),
            "done" => Ok(Command::Done { id: arg()? })
            _ => Err(AppError::InvalidCommand(String::from(command))),
        }
    }
}

pub fn run() -> error::Result<()> {
    let stdin = std::io::stdin();

    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;
    }

    Ok(())
}
