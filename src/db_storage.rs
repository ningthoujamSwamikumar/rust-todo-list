use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::{
    error::TodoError,
    todo_list::{Todo, TodoOps, TodoResult},
};

#[derive(Debug)]
pub struct DbStorage {
    executor: Pool<Postgres>,
}

impl DbStorage {
    pub fn new(executor: Pool<Postgres>) -> Self {
        Self { executor }
    }
}

#[async_trait]
impl TodoOps for DbStorage {
    async fn add(&mut self, value: String) -> Result<TodoResult, TodoError> {
        let result: Todo =
            sqlx::query_as::<_, Todo>("INSERT INTO todo (task) VALUES ($1) RETURNING id, task")
                .bind(value)
                .fetch_one(&self.executor)
                .await?;
        Ok(TodoResult::Added(result))
    }

    async fn update(&mut self, index: i32, value: String) -> Result<TodoResult, TodoError> {
        let result: Todo =
            sqlx::query_as::<_, Todo>("UPDATE todo SET task=$1 WHERE id=$2 RETURNING id, task")
                .bind(value)
                .bind(index)
                .fetch_one(&self.executor)
                .await?;

        Ok(TodoResult::Updated(result))
    }

    async fn delete(&mut self, index: i32) -> Result<TodoResult, TodoError> {
        let result: Todo =
            sqlx::query_as::<_, Todo>("DELETE FROM todo WHERE id=$1 RETURNING id, task")
                .bind(index)
                .fetch_one(&self.executor)
                .await?;

        Ok(TodoResult::Deleted(result))
    }

    async fn get(&self, index: i32) -> Result<TodoResult, TodoError> {
        let result: Todo = sqlx::query_as::<_, Todo>("SELECT * FROM todo WHERE id=$1")
            .bind(index)
            .fetch_one(&self.executor)
            .await?;

        Ok(TodoResult::Gotten(result))
    }

    async fn get_all(&self) -> Result<TodoResult, TodoError> {
        let result: Vec<Todo> = sqlx::query_as("SELECT * FROM todo")
            .fetch_all(&self.executor)
            .await?;

        Ok(TodoResult::GottenAll(result))
    }

    async fn clear(&mut self) -> Result<TodoResult, TodoError> {
        let result = sqlx::query("TRUNCATE TABLE todo")
            .execute(&self.executor)
            .await?;
        println!("Cleared {} rows", result.rows_affected());

        Ok(TodoResult::Cleared)
    }
}
