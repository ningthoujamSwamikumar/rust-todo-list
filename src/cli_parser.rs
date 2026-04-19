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
        value: String,
    },
    /// Delete a task at index
    Delete {
        /// The index of the task to be deleted
        #[arg()]
        index: usize,
    },
    /// Show a task or all task
    Show {
        /// The index of the task to be shown, if None all tasks are shown
        #[arg()]
        index: Option<usize>,
    },
    /// Update a task at an index
    Update {
        /// The index of the task to be updated
        #[arg(long, short)]
        index: usize,
        /// The new task description
        #[arg(long, short)]
        value: String,
    },
    /// Clear all the tasks in the todo list
    Clear,
}
