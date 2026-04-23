use std::path::Path;

use clap::Parser;

use crate::{error::TodoError, todo_list::TodoList};

pub mod cli_parser;
pub mod error;
pub mod todo_list;

#[cfg(test)]
pub mod tests;

/// Initializes a todo list which stores in a file.
/// This function takes inputs from command line
pub async fn init_file_todo() {
    let cli = cli_parser::Cli::parse();

    let path = Path::new("target/.todo_list.json");

    let mut todo_list = match TodoList::from_file(path) {
        Ok(list) => list,
        Err(e) => {
            println!("{:?}", e);
            TodoList::new()
        }
    };

    let mut retrievals = Vec::<String>::new();
    match action_handler(cli.action, &mut todo_list, &mut retrievals).await {
        Ok(_) => println!("Action Completed Successfully"),
        Err(e) => eprintln!("Error performing action:\n{:?}", e),
    };

    //println!("retrievals:\n{:?}", retrievals);

    match todo_list.write_file(path) {
        Ok(_) => println!("Written to file"),
        Err(e) => println!("{:?}", e),
    }
}

pub async fn action_handler(
    action: cli_parser::Actions,
    todo_list: &mut TodoList,
    buf: &mut Vec<String>,
) -> Result<(), TodoError> {
    match action {
        cli_parser::Actions::Add { value } => todo_list.add(value).await,
        cli_parser::Actions::Delete { index } => todo_list.delete(index).await,
        cli_parser::Actions::Show { index } => match index {
            Some(i) => {
                let mut buf_0 = String::new();
                todo_list.get(i, &mut buf_0).await?;
                buf.push(buf_0);
                Ok(())
            }
            None => todo_list.get_all(buf).await,
        },
        cli_parser::Actions::Update { index, value } => todo_list.update(index, value).await,
        cli_parser::Actions::Clear => todo_list.clear().await,
    }
}
