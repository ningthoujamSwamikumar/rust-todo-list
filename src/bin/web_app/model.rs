use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Todo {
    pub id: i32,
    pub task: String,
    pub state: TodoState,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub enum TodoState {
    Pending,
    InProgress,
    Finished,
}
