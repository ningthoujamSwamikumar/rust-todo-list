use clap::Parser;
use std::path::Path;

use rust_todo_list::{
    action_handler, cli_parser,
    file_storage::{self, FileStorage},
};

/// This function performs operation through cli arguments, and <br>
/// the list is store in file. This doesn't use client-sever architecture
#[tokio::main]
async fn main() {
    let cli = cli_parser::Cli::parse();

    let path = Path::new("target/.todo_list.json");

    let mut todo_list = match file_storage::FileStorage::from_file(path) {
        Ok(list) => list,
        Err(e) => {
            println!("{:?}", e);
            FileStorage::new()
        }
    };

    let mut retrievals = Vec::<String>::new();
    match action_handler(cli.action, &mut todo_list, &mut retrievals).await {
        Ok(_) => println!("Action Completed Successfully"),
        Err(e) => eprintln!("Error performing action:\n{:?}", e),
    };

    println!("retrievals:\n{:?}", retrievals);

    match todo_list.write_file(path) {
        Ok(_) => println!("Written to file"),
        Err(e) => println!("{:?}", e),
    }
}
