use std::fmt::Display;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::TodoError;

#[derive(Debug, Clone, PartialEq, PartialOrd, sqlx::FromRow, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub task: String,
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ id: {}, task: {} }}", self.id, self.task)
    }
}

#[derive(Debug, Serialize)]
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

#[async_trait]
pub trait TodoOps {
    async fn add(&mut self, task: String) -> Result<TodoResult, TodoError>;

    async fn update(&mut self, id: i32, task: String) -> Result<TodoResult, TodoError>;

    async fn delete(&mut self, id: i32) -> Result<TodoResult, TodoError>;

    async fn get(&self, id: i32) -> Result<TodoResult, TodoError>;

    async fn get_all(&self) -> Result<TodoResult, TodoError>;

    async fn clear(&mut self) -> Result<TodoResult, TodoError>;
}
