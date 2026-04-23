use std::{
    fmt::Write,
    io::{self, BufReader},
    path::Path,
};

use serde::{Deserialize, Serialize};
use sqlx::{
    Pool, Postgres, Row,
    postgres::{PgQueryResult, PgRow},
};

use crate::error::TodoError;

/// Struct that holds the list in memory when using files to persist the list
/// When using db, we just initialized to empty list, and not really use the contents
#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    contents: Vec<String>,
    #[serde(skip)]
    executor: Option<Pool<Postgres>>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            contents: vec![],
            executor: Option::default(),
        }
    }

    pub fn new_with_db(executor: Pool<Postgres>) -> Self {
        TodoList {
            contents: vec![],
            executor: Some(executor),
        }
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
    pub async fn add(&mut self, value: String) -> Result<(), TodoError> {
        if let Some(pool) = &self.executor {
            let result: PgQueryResult = sqlx::query("INSERT INTO todo (task) VALUES ($1)")
                .bind(value)
                .execute(pool)
                .await?;
            println!("Inserted {} row", result.rows_affected());
            return Ok(());
        }

        self.contents.push(value);
        println!("Value added at {}", self.contents.len() - 1);
        Ok(())
    }

    pub async fn update(&mut self, index: usize, value: String) -> Result<(), TodoError> {
        if let Some(pool) = &self.executor {
            // task when stored in a db
            let result: PgQueryResult = sqlx::query("UPDATE todo SET task=$1 WHERE id=$2")
                .bind(value)
                .bind(index as i32)
                .execute(pool)
                .await?;
            println!("Updated {} rows.", result.rows_affected());

            return Ok(());
        }

        // processing when task are stored in file
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

    pub async fn delete(&mut self, index: usize) -> Result<(), TodoError> {
        if let Some(pool) = &self.executor {
            let result: PgQueryResult = sqlx::query("DELETE FROM todo WHERE id=$1")
                .bind(index as i32)
                .execute(pool)
                .await?;
            println!("Deleted {} row.", result.rows_affected());

            return Ok(());
        }

        if index >= self.contents.len() {
            return Err(TodoError::InvalidInput("Invalid index provided!".into()));
        };
        self.contents.remove(index);
        println!("Value removed from index {}", index);
        Ok(())
    }

    pub async fn get(&self, index: usize, buf: &mut String) -> Result<(), TodoError> {
        if let Some(pool) = &self.executor {
            let result: PgRow = sqlx::query("SELECT * FROM todo WHERE id=$1")
                .bind(index as i32)
                .fetch_one(pool)
                .await?;
            buf.write_fmt(format_args!(
                "{}, \"{}\"",
                result.get::<i32, &str>("id"),
                result.get::<String, &str>("task")
            ))?;

            return Ok(());
        }

        if let Some(value) = self.contents.get(index) {
            buf.write_str(value)
                .map_err(|e| TodoError::FailedToWrite(e.to_string()))?;
            Ok(())
        } else {
            Err(TodoError::InvalidInput("Invalid index provided!".into()))
        }
    }

    pub async fn get_all(&self, buf: &mut Vec<String>) -> Result<(), TodoError> {
        if let Some(pool) = &self.executor {
            let result: Vec<PgRow> = sqlx::query("SELECT * FROM todo").fetch_all(pool).await?;

            for row in result {
                buf.push(format!(
                    "({},\"{}\")",
                    row.get::<i32, &str>("id"),
                    row.get::<String, &str>("task")
                ));
            }

            return Ok(());
        }

        self.contents
            .iter()
            .for_each(|v: &String| buf.push(v.clone()));
        Ok(())
    }

    pub async fn clear(&mut self) -> Result<(), TodoError> {
        if let Some(pool) = &self.executor {
            let result: PgQueryResult = sqlx::query("TRUNCATE TABLE todo").execute(pool).await?;
            println!("Cleared {} rows", result.rows_affected());

            return Ok(());
        }

        self.contents.clear();
        Ok(())
    }
}
