use std::io::{self, Write};

fn main() {
    let mut todo_list: Vec<String> = Vec::<String>::new();

    let mut input = String::new();

    loop {
        input.clear(); // This allows us to reuse the allocated buffer in case of continuous loop
        // Show instructions to the user
        println!("\n----------------------------------");
        println!("Todo list: \n{:?}", todo_list);
        println!("====Actions Available:====");
        println!("SHW \t ADD \t UPD \t DEL \t EXT");
        println!(
            "Info - Provide the input in the format \"[ Action ] - [ index when required ] - [ input when required ]\""
        );
        println!("Example input: \"ADD - wash clothes\"");
        println!("Example input: \"SHW\"");
        println!();
        print!("Enter:\t"); // This writes to stdout but doesn't flush automatically like println! 
        io::stdout().flush().unwrap(); // This flushes the stdout buffer

        // Read the inputs provided by the user
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read input from stdin");

        // Handle the input according to the Command provided
        let inputs: Vec<&str> = input.trim().split("-").map(|s| s.trim()).collect();

        // Validate input components' count
        if inputs.len() < 1 {
            println!("Expected 1 or more input components!");
            continue;
        }
        // Extract the command component
        let command = inputs.get(0);
        // Validate and execute commands
        let Some(&cmd) = command else {
            println!("Invalid Command!");
            continue;
        };

        match cmd {
            "SHW" => {
                println!("Todo list:\n{:?}", todo_list);
            }
            "ADD" => {
                let Some(&value) = inputs.get(1) else {
                    println!("Expected a value to add!");
                    continue;
                };
                todo_list.push(value.to_string());
                println!(
                    "Added at {}. [**The index might change later.]",
                    todo_list.len() - 1
                );
            }
            "UPD" => {
                let (Some(&index), Some(&content)) = (inputs.get(1), inputs.get(2)) else {
                    println!(
                        "Expected input format - \"[ Action ] - [ index ] - [ new_content ]\""
                    );
                    continue;
                };

                let Ok(index) = index.parse::<usize>() else {
                    println!("Failed to parse the index!");
                    continue;
                };

                if index >= todo_list.len() {
                    println!("Index out of bound!");
                    continue;
                }

                let Some(value) = todo_list.get_mut(index) else {
                    println!("Unable to access the element at index!");
                    continue;
                };

                *value = content.to_string();
                println!("Updated value at {}", index);
            }
            "DEL" => {
                let Some(&index) = inputs.get(1) else {
                    println!("Expected input format - \"[ Action ] - [ index ]\"");
                    continue;
                };
                let Ok(index) = index.parse::<usize>() else {
                    println!("Failed to parse the index!");
                    continue;
                };

                if index >= todo_list.len() {
                    println!("Index out of bound!");
                    continue;
                }

                todo_list.remove(index);
                println!("Value removed from index: {}", index);
            }
            "EXT" => {
                println!("Exit command");
                break;
            }
            _ => {
                println!("Invalid command!");
                continue;
            }
        }
    }
}
