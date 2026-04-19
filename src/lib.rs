use std::path::Path;

use clap::Parser;

use crate::todo_list::{TodoError, TodoList};

pub mod cli_parser;
pub mod todo_list;

pub fn init_handler() {
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
    match action_handler(&cli.action, &mut todo_list, &mut retrievals) {
        Ok(_) => println!("Action Completed Successfully"),
        Err(e) => eprintln!("Error performing action:\n{:?}", e),
    };

    println!("retrievals:\n{:?}", retrievals);

    match cli.action {
        cli_parser::Actions::Show { index: _ } => println!("Read operation need no file update."),
        _ => match todo_list.write_file(path) {
            Ok(_) => println!("Written to file"),
            Err(e) => println!("{:?}", e),
        },
    };
}

pub fn action_handler(
    action: &cli_parser::Actions,
    todo_list: &mut TodoList,
    buf: &mut Vec<String>,
) -> Result<(), TodoError> {
    match action {
        cli_parser::Actions::Add { value } => todo_list.add(value.clone()),
        cli_parser::Actions::Delete { index } => todo_list.delete(index.clone()),
        cli_parser::Actions::Show { index } => match index {
            Some(i) => {
                let mut buf_0 = String::new();
                todo_list.get(i.clone(), &mut buf_0)?;
                buf.push(buf_0);
                Ok(())
            }
            None => todo_list.get_all(buf),
        },
        cli_parser::Actions::Update { index, value } => {
            todo_list.update(index.clone(), value.clone())
        }
        cli_parser::Actions::Clear => todo_list.clear(),
    }
}
