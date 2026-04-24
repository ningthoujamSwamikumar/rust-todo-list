use std::fmt::Display;

use crate::error::TodoError;

#[derive(Debug, Clone, PartialEq, PartialOrd, sqlx::FromRow)]
pub struct Todo {
    pub id: i32,
    pub task: String,
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ id: {}, task: {} }}", self.id, self.task)
    }
}

#[derive(Debug)]
pub enum TodoResult {
    Cleared,
    Added(Todo),
    Updated(Todo),
    Deleted(Todo),
    Gotten(Todo),
    GottenAll(Vec<Todo>),
}

impl Display for TodoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoResult::Cleared => write!(f, "Cleared"),
            TodoResult::Added(todo) => write!(f, "Added {}", todo),
            TodoResult::Updated(todo) => write!(f, "Updated {}", todo),
            TodoResult::Deleted(todo) => write!(f, "Deleted {}", todo),
            TodoResult::Gotten(todo) => write!(f, "Fetched {}", todo),
            TodoResult::GottenAll(list) => write!(f, "Fetched:\n{:?}", list),
        }
    }
}

pub trait TodoOps {
    fn add(&mut self, value: String) -> impl Future<Output = Result<TodoResult, TodoError>>;

    fn update(
        &mut self,
        index: i32,
        value: String,
    ) -> impl Future<Output = Result<TodoResult, TodoError>>;

    fn delete(&mut self, index: i32) -> impl Future<Output = Result<TodoResult, TodoError>>;

    fn get(&self, index: i32) -> impl Future<Output = Result<TodoResult, TodoError>>;

    fn get_all(&self) -> impl Future<Output = Result<TodoResult, TodoError>>;

    fn clear(&mut self) -> impl Future<Output = Result<TodoResult, TodoError>>;
}
