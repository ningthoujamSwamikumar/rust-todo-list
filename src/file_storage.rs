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
    contents: Vec<String>,
}

impl FileStorage {
    pub fn new() -> Self {
        FileStorage { contents: vec![] }
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
    async fn add(&mut self, value: String) -> Result<TodoResult, TodoError> {
        self.contents.push(value.clone());
        let i = self.contents.len() - 1;

        println!("Value added at {}", i);

        Ok(TodoResult::Added(Todo {
            id: i as i32,
            task: value,
        }))
    }

    async fn update(&mut self, index: i32, value: String) -> Result<TodoResult, TodoError> {
        let index = index as usize;
        // processing when task are stored in file
        if index >= self.contents.len() {
            return Err(TodoError::InvalidInput("Invalid index provided!".into()));
        };
        let Some(element) = self.contents.get_mut(index) else {
            return Err(TodoError::AccessError(
                "Failed to access the element at index!".into(),
            ));
        };
        *element = value.clone();
        println!("Value updated at index {}", index);

        Ok(TodoResult::Updated(Todo {
            id: index as i32,
            task: value,
        }))
    }

    async fn delete(&mut self, index: i32) -> Result<TodoResult, TodoError> {
        let index = index as usize;
        if index >= self.contents.len() {
            return Err(TodoError::InvalidInput("Invalid index provided!".into()));
        };
        let task = self.contents.remove(index);

        println!("Value removed from index {}", index);

        Ok(TodoResult::Deleted(Todo {
            id: index as i32,
            task,
        }))
    }

    async fn get(&self, index: i32) -> Result<TodoResult, TodoError> {
        if let Some(value) = self.contents.get(index as usize) {
            Ok(TodoResult::Gotten(Todo {
                id: index,
                task: value.clone(),
            }))
        } else {
            Err(TodoError::InvalidInput("Invalid index provided!".into()))
        }
    }

    async fn get_all(&self) -> Result<TodoResult, TodoError> {
        let todos: Vec<Todo> = self
            .contents
            .iter()
            .enumerate()
            .map(|(i, v)| Todo {
                id: i as i32,
                task: v.clone(),
            })
            .collect();

        Ok(TodoResult::GottenAll(todos))
    }

    async fn clear(&mut self) -> Result<TodoResult, TodoError> {
        self.contents.clear();
        Ok(TodoResult::Cleared)
    }
}
