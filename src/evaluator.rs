use crate::error;
use crate::parser::Command;
use crate::run::TodoList;

pub enum Flow {
    Continue,
    Exit,
}

/// Applies `command` to `list` and tells the caller whether to keep running.
///
/// # Errors
///
/// Returns [`error::AppError::TodoNotFound`] if `remove` or `done` names a todo
/// that doesn't exist.
pub fn eval(list: &mut TodoList, command: Command) -> error::Result<Flow> {
    match command {
        Command::Add { name } => list.add(name),
        Command::Remove { name } => {
            let id = list.find_by_name(name)?;
            list.remove(id);
        }
        Command::Done { name } => {
            let id = list.find_by_name(name)?;
            list.done(id, true)?;
        }
        Command::List => {
            println!("{list}");
        }
        Command::Exit => return Ok(Flow::Exit),
    }

    Ok(Flow::Continue)
}
