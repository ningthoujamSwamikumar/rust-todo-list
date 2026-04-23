use std::fmt::Display;

#[derive(Debug)]
pub enum TodoError {
    Sqlx(sqlx::Error),
    Io(std::io::Error),
    InvalidInput(String),
    AccessError(String),
    FailedToWrite(String),
    DbConnectionError(String),
    Fmt(std::fmt::Error),
}

impl Display for TodoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoError::AccessError(err) => write!(f, "AccessError - {}", err),
            TodoError::FailedToWrite(err) => write!(f, "FailedToWrite - {}", err),
            TodoError::InvalidInput(err) => write!(f, "InvalidInput - {}", err),
            TodoError::DbConnectionError(err) => write!(f, "DbConnectionError - {}", err),
            TodoError::Io(err) => write!(f, "Io - {}", err.to_string()),
            TodoError::Sqlx(err) => write!(f, "Sqlx - {}", err.to_string()),
            TodoError::Fmt(err) => write!(f, "std::fmt::Error - {}", err.to_string()),
        }
    }
}

impl From<sqlx::Error> for TodoError {
    fn from(value: sqlx::Error) -> Self {
        TodoError::Sqlx(value)
    }
}

impl From<std::io::Error> for TodoError {
    fn from(value: std::io::Error) -> Self {
        TodoError::Io(value)
    }
}

impl From<std::fmt::Error> for TodoError {
    fn from(value: std::fmt::Error) -> Self {
        TodoError::Fmt(value)
    }
}
