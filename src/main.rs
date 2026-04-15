use std::path::Path;

use clap::Parser;

use crate::todo_list::TodoList;

mod cli_parser;
mod todo_list;

fn main() {
    let cli = cli_parser::Cli::parse();

    let path = Path::new("target/.todo_list.json");

    let mut todo_list = match TodoList::from_file(path) {
        Ok(list) => list,
        Err(e) => {
            println!("{:?}", e);
            TodoList::new()
        }
    };

    //println!("Todo list:\n{:?}", todo_list);

    match cli.action {
        cli_parser::Actions::Add { value } => match todo_list.add(value) {
            Ok(_) => println!("Action completed"),
            Err(e) => println!("{:?}", e),
        },
        cli_parser::Actions::Delete { index } => match todo_list.delete(index) {
            Ok(_) => println!("Action completed"),
            Err(e) => println!("{:?}", e),
        },
        cli_parser::Actions::Show { index } => match index {
            Some(i) => match todo_list.get(i) {
                Ok(s) => {
                    println!("{}", s);
                    println!("Action completed")
                }
                Err(e) => println!("{:?}", e),
            },
            None => match todo_list.get_all() {
                Ok(list) => {
                    println!("{:?}", list);
                    println!("Action completed")
                }
                Err(e) => println!("{:?}", e),
            },
        },
        cli_parser::Actions::Update { index, value } => match todo_list.update(index, value) {
            Ok(_) => println!("Action completed"),
            Err(e) => println!("{:?}", e),
        },
        cli_parser::Actions::Clear => match todo_list.clear() {
            _ => println!("Action completed"),
        },
    };

    match todo_list.write_file(path) {
        Ok(_) => println!("Written to file"),
        Err(e) => println!("{:?}", e),
    };
}
