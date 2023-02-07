#![warn(clippy::all)]
//use std::io;
use std::io::{self, Write};

enum MenuOption {
    AddBill,
    ViewBills,
}

impl MenuOption {
    fn new(option: &str) -> Option<MenuOption> {
        let option = option.trim();
        match option {
            "1" => Some(MenuOption::AddBill),
            "2" => Some(MenuOption::ViewBills),
            _ => None,
        }
    }
}

fn print_menu_choice(choice: MenuOption) {
    use MenuOption::*;
    match choice {
        AddBill => println!("Add bill chosen"),
        ViewBills => println!("View bills chosen"),
    }
}

fn main() {
    let mut buffer = String::new();

    println!(
        "
    == Manage Bills ==
    1. Add bill
    2. View bills\n
    Enter selection:"
    );
    print!("> ");

    let get_input = io::stdin().read_line(&mut buffer);
    let _ = io::stdout().flush();
    //let answer = buffer.trim();

    if get_input.is_ok() {
        match MenuOption::new(&buffer) {
            Some(buffer) => print_menu_choice(buffer),
            None => println!("Invalid option"),
        }
    } else {
        println!("error")
    }
}
