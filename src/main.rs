use std::{collections::HashMap, io};

fn main() {
    let departments = vec!["Sales", "Engineering"];
    let mut users_map = HashMap::new();

    loop {
        println!("Type 1 to add user or 2 to search for users or 3 to quit.");

        let mut direction = String::new();

        io::stdin()
            .read_line(&mut direction)
            .expect("Failed to read line");

        let value: i32 = match direction.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("The value is {}", &value);

        //newline to separate the input from the output
        println!();
        println!();

        if value == 1 {
            println!("Enter the user name:");
            let mut user_name = String::new();
            io::stdin()
                .read_line(&mut user_name)
                .expect("Failed to read line");

            println!("Enter the department:");
            let mut department = String::new();
            io::stdin()
                .read_line(&mut department)
                .expect("Failed to read line");

            // let user = user_name.trim();
            // let department_user = department.trim();

            if departments.contains(&department.trim()) {
                println!(
                    "The user {} has been added to the department {}",
                    user_name, department
                );
                users_map.insert(department, user_name);
            } else {
                println!("The '{}' department does not exist", department);
            }
        } else if value == 2 {
            println!("To search by department type 1. To search by user name type 2");

            println!();

            let mut search_type = String::new();
            io::stdin()
                .read_line(&mut search_type)
                .expect("Failed to read search type");

            let value: i32 = match search_type.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            if value == 1 {
                println!("Enter the department:");
                let mut department = String::new();
                io::stdin()
                    .read_line(&mut department)
                    .expect("Failed to read department");

                //search hashmap by department

                for (k, v) in &users_map {
                    println!("Users in {}", department);
                    if k.eq_ignore_ascii_case(&department) {
                        println!("* {v}");
                    };
                }
            } else if value == 2 {
                println!("Enter users username:");
                println!();
                let mut user_name = String::new();
                io::stdin()
                    .read_line(&mut user_name)
                    .expect("Failed to read Username");

                //search hashmap by department

                for (k, v) in &users_map {
                    if v.eq_ignore_ascii_case(&user_name) {
                        println!("* {k}");
                    };
                }
            }
        } else if value == 3 {
            break;
        } else {
            println!("Invalid input");
        }

        // println!("The user map is {:#?}",&users_map);
    }
}
