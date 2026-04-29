use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(name = "todo", version = "1.0", about = "Manage your tasks")]
pub struct Cli {
    /// Subcommands
    #[command(subcommand)]
    pub action: Actions,
}

#[derive(Debug, Subcommand, Serialize, Deserialize)]
pub enum Actions {
    /// Add a new task
    Add {
        /// The task description
        #[arg()]
        task: String,
    },
    /// Delete a task at id
    Delete {
        /// The id of the task to be deleted
        #[arg()]
        id: i32,
    },
    /// Show a task or all task
    Show {
        /// The id of the task to be shown, if None all tasks are shown
        #[arg()]
        id: Option<i32>,
    },
    /// Update a task at an id
    Update {
        /// The id of the task to be updated
        #[arg(long, short)]
        id: i32,
        /// The new task description
        #[arg(long, short)]
        task: String,
    },
    /// Clear all the tasks in the todo list
    Clear,
}
