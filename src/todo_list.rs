use std::{
    io::{self, BufReader},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct TodoList {
    contents: Vec<String>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { contents: vec![] }
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

/// define operations on todo_list
impl TodoList {
    pub fn add(&mut self, value: String) -> Result<(), TodoError> {
        self.contents.push(value);
        println!("Value added at {}", self.contents.len() - 1);
        Ok(())
    }

    pub fn update(&mut self, index: usize, value: String) -> Result<(), TodoError> {
        if index >= self.contents.len() {
            return Err(TodoError::InvalidInput("Invalid index provided!".into()));
        };
        let Some(element) = self.contents.get_mut(index) else {
            return Err(TodoError::AccessError(
                "Failed to access the element at index!".into(),
            ));
        };
        *element = value;
        println!("Value updated at index {}", index);
        Ok(())
    }

    pub fn delete(&mut self, index: usize) -> Result<(), TodoError> {
        if index >= self.contents.len() {
            return Err(TodoError::InvalidInput("Invalid index provided!".into()));
        };
        self.contents.remove(index);
        println!("Value removed from index {}", index);
        Ok(())
    }

    pub fn get(&self, index: usize) -> Result<String, TodoError> {
        if let Some(value) = self.contents.get(index) {
            Ok(value.clone())
        } else {
            Err(TodoError::InvalidInput("Invalid index provided!".into()))
        }
    }

    pub fn get_all(&self) -> Result<Vec<String>, TodoError> {
        Ok(self.contents.clone())
    }

    pub fn clear(&mut self) -> Result<(), TodoError> {
        self.contents.clear();
        Ok(())
    }
}

#[derive(Debug)]
pub(super) enum TodoError {
    //ParseError(String),
    InvalidInput(String),
    AccessError(String),
}
