use crate::error;
use crate::evaluator::{Flow, eval};
use crate::parser::Command;
use std::io::Write;
use std::{fmt, io};
use uuid::Uuid;

struct Todo {
    id: Uuid,
    name: String,
    done: bool,
}

impl Todo {
    pub fn new(name: String) -> Self {
        Todo {
            id: Uuid::new_v4(),
            name,
            done: false,
        }
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mark = if self.done { "✅" } else { "☐" };
        write!(f, "{mark} {}", self.name)
    }
}
pub struct TodoList {
    todos: Vec<Todo>,
}

impl fmt::Display for TodoList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.todos.is_empty() {
            return write!(f, "No todos yet.");
        }

        for (i, todo) in self.todos.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{todo}")?;
        }
        Ok(())
    }
}

impl Default for TodoList {
    fn default() -> Self {
        Self::new()
    }
}

impl TodoList {
    #[must_use]
    pub fn new() -> Self {
        TodoList { todos: Vec::new() }
    }

    pub fn add(&mut self, name: String) {
        self.todos.push(Todo::new(name));
    }

    pub fn remove(&mut self, id: Uuid) {
        self.todos.retain(|todo| todo.id != id);
    }

    /// Marks the todo with the given `id` as done or not done.
    ///
    /// # Panics
    ///
    /// Panics if no todo with `id` exists. Get the id from [`TodoList::find_by_name`].
    pub fn done(&mut self, id: Uuid, is_done: bool) {
        let todo = self
            .todos
            .iter_mut()
            .find(|todo| todo.id == id)
            .expect("Caller should garantee existence");
        todo.done = is_done;
    }

    /// Returns the id of the first todo named `name`.
    ///
    /// # Errors
    ///
    /// Returns [`error::AppError::TodoNotFound`] if no todo has that name.
    pub fn find_by_name(&self, name: String) -> error::Result<Uuid> {
        let todo = self.todos.iter().find(|todo| todo.name == name);
        match todo {
            Some(todo) => Ok(todo.id),
            None => Err(error::AppError::TodoNotFound(name)),
        }
    }

    pub fn list(&self) {
        for todo in &self.todos {
            println!("{todo}");
        }
    }
}

/// Runs the interactive todo prompt until `exit` is entered.
///
/// # Errors
///
/// Returns an error if reading stdin or flushing stdout fails. Invalid commands
/// and unknown todos are printed to stderr and don't stop the loop.
pub fn run() -> error::Result<()> {
    let mut todo_list = TodoList::new();
    let stdin = std::io::stdin();

    loop {
        print!("todo > ");
        io::stdout().flush()?;

        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;

        match buffer
            .trim()
            .parse::<Command>()
            .and_then(|command| eval(&mut todo_list, command))
        {
            Ok(Flow::Exit) => break,
            Ok(Flow::Continue) => {}
            Err(e) => eprintln!("{e}"),
        }
    }

    Ok(())
}
