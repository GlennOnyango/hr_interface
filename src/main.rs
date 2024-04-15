use std::{collections::HashMap, io};

fn main() {
    let departments = vec!["Sales", "Engineering"];
    // let users_map = HashMap::new();

    loop {
        println!("Select 1 to add user or 2 to search for users or 3 to quit.");

        let mut direction = String::new();

        io::stdin()
            .read_line(&mut direction)
            .expect("Failed to read line");

        let exit = '3'.to_string();
        let search = '2'.to_string();
        let add = '1'.to_string();

        let status = match direction.as_str() {
            exit => "exit",
            search => "search",
            add => "add",
            _ => "continue",
        };

        if status.eq("exit") {
            break;
        } else if status.eq("search") {
        } else if status.eq("add") {
            let mut user_name = String::new();
            println!("Enter user name");
            io::stdin()
                .read_line(&mut user_name)
                .expect("Failed to read user name");

            println!()
        }

        println!("The status {}", status);
    }
}
