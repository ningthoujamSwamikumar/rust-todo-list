use std::{
    io::{self, BufReader},
    path::Path,
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    error::TodoError,
    todo_list::{Todo, TodoOps, TodoResult},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileStorage {
    contents: Vec<Todo>,
    counter: i32,
}

impl FileStorage {
    pub fn new() -> Self {
        FileStorage {
            contents: vec![],
            counter: 0,
        }
    }

    fn get_counter(&mut self) -> i32 {
        let id = self.counter;
        self.counter += 1;
        id
    }

    /// Read todo list from a json file
    pub fn from_file(path: &Path) -> io::Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = BufReader::new(file);

        let todo_list: Self = serde_json::from_reader(reader)?;
        Ok(todo_list)
    }

    pub fn write_file(&self, path: &Path) -> io::Result<()> {
        //validate it is a json file
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => {
                let file = std::fs::File::create(path)?;
                serde_json::to_writer_pretty(file, self)?;
                Ok(())
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Expected a Json file",
            )),
        }
    }
}

#[async_trait]
impl TodoOps for FileStorage {
    async fn add(&mut self, task: String) -> Result<TodoResult, TodoError> {
        let todo = Todo {
            id: self.get_counter(),
            task: task,
        };
        self.contents.push(todo.clone());

        println!("Value added at {}", &todo.id);

        Ok(TodoResult::Added(todo))
    }

    async fn update(&mut self, id: i32, task: String) -> Result<TodoResult, TodoError> {
        let Some(todo) = self.contents.iter_mut().find(|v| v.id == id) else {
            return Err(TodoError::InvalidInput("Todo not found!".into()));
        };

        todo.task = task;
        println!("Value updated at index {}", id);

        Ok(TodoResult::Updated(todo.clone()))
    }

    async fn delete(&mut self, id: i32) -> Result<TodoResult, TodoError> {
        let Some(pos) = self.contents.iter().position(|v| v.id == id) else {
            return Err(TodoError::InvalidInput("Todo not found!".into()));
        };

        let todo = self.contents.remove(pos);

        println!("Value removed having id: {}", id);

        Ok(TodoResult::Deleted(todo))
    }

    async fn get(&self, id: i32) -> Result<TodoResult, TodoError> {
        let Some(todo) = self.contents.iter().find(|v| v.id == id) else {
            return Err(TodoError::InvalidInput("Invalid index provided!".into()));
        };
        Ok(TodoResult::Gotten(todo.clone()))
    }

    async fn get_all(&self) -> Result<TodoResult, TodoError> {
        Ok(TodoResult::GottenAll(self.contents.clone()))
    }

    async fn clear(&mut self) -> Result<TodoResult, TodoError> {
        self.contents.clear();
        Ok(TodoResult::Cleared)
    }
}
