use std::io;

fn main() {
    let mut todo_list: Vec<String> = Vec::<String>::new();

    let mut input = String::new();

    println!("Add your to do list:");

    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read input from stdin");

    todo_list.push(input.trim().to_string());
    println!("Todo inserted");
    println!("Todo list:\n{:?}", todo_list);
}
