#![warn(clippy::all)]
//use std::io;
use std::io::{self, Write};

pub mod menu {
    pub enum MenuOption {
        AddBill,
        ViewBills,
    }

    impl MenuOption {
        pub fn new(option: &str) -> Option<MenuOption> {
            let option = option.trim();
            match option {
                "1" => Some(MenuOption::AddBill),
                "2" => Some(MenuOption::ViewBills),
                _ => None,
            }
        }
    }
    pub fn menu_choice(choice: MenuOption) {
        use crate::add_bill;
        match choice {
            MenuOption::AddBill => add_bill(),
            MenuOption::ViewBills => println!("View bills chosen"),
        }
    }
}

pub mod add {
    #[derive(Debug)]
    pub struct Add {
        name: String,
        amount_owned: f64,
    }

    impl Add {
        pub fn new(name: &str, amount_owned: f64) -> Add {
            Add {
                name: name.to_string(),
                amount_owned,
            }
        }
    }
}

fn add_bill() {
    // Reference: https://users.rust-lang.org/t/how-to-read-an-integer-from-stdin/57538/2
    use add::*;
    let mut buffer = String::new();
    println!("Name:");
    print!("> ");
    io::stdin()
        .read_line(&mut buffer)
        .expect("Expected a string as input");
    let _ = io::stdout().flush();
    let name = buffer.trim();

    let mut number_buffer = String::new();
    println!("Amount owned:");
    print!("> ");
    io::stdin()
        .read_line(&mut number_buffer)
        .expect("Expected a floating number as input");
    let _ = io::stdout().flush();
    let amount: f64 = number_buffer.trim().parse().expect("Expected a number");

    let add = Add::new(name, amount);
    println!("{:?}", add);
}

fn main() {
    use menu::*;
    // Create an empty string
    let mut buffer = String::new();

    println!(
        "
    == Manage Bills ==
    1. Add bill
    2. View bills\n
    Enter selection:"
    );
    print!("> ");

    // Get user input
    let get_input = io::stdin().read_line(&mut buffer);
    let _ = io::stdout().flush();
    //let answer = buffer.trim();

    if get_input.is_ok() {
        match MenuOption::new(&buffer) {
            Some(buffer) => menu_choice(buffer),
            None => println!("Invalid option"),
        }
    } else {
        println!("Error")
    }
}
