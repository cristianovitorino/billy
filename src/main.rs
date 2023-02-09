//use std::collections::HashMap;
use std::io;

/// The bill template
/// 'Debug' to easily print on terminal and 'Clone' to make copies of the struct
#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}

/// 'Bills' superstructure, the "storage"
pub struct Bills {
    /// vec![{name: String, amount: f64}]
    inner: Vec<Bill>,
}

/// 'Bills' "storage" superstructre implementation
impl Bills {
    /// Instanciate a new 'Bills' struct with an empty vector
    fn new() -> Self {
        Self { inner: vec![] }
    }

    /// Add a new bill
    /// '&mut self' access mutably the 'inner: Vec<Bill>' on the 'Bills' struct
    /// The owned 'Bill' moves into the 'Bills' struct 'Vec<Bill>'
    /// Push the new bill into the vector
    fn add(&mut self, bill: Bill) {
        self.inner.push(bill);
    }

    /// Print the bills
    /// Return a reference to the vector
    /// Reference to itself '&self' so it's able to access the 'Bills' struct
    /// 'iter()' over all bills, automaticaly borrows
    /// 'collect()' collects into a new vector
    fn get_all_bills(&self) -> Vec<&Bill> {
        self.inner.iter().collect()
    }
}

/// Get the user input data as a 'String'
fn get_input() -> Option<String> {
    // Empty 'String' template
    let mut buffer = String::new();

    // Loop until gets valid data
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again...");
    }

    // Get rid of newline(\n) at the end using 'trim()' and make the 'String' owned (because 'buffer' is a 'String')
    let input = buffer.trim().to_owned();

    // If empty, return nothing
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

/// Transform the 'String' bill amount input into a 'f64'
fn get_amount_as_float() -> Option<f64> {
    println!("Amount:");

    // 'return' is used to capture the desired result and get out of the loop
    loop {
        // Get input
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };

        // Get out if none
        if &input == "" {
            return None;
        }

        // Parse the input string into a float
        // Let Rust figure out the error type since it's not relevant
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Invalid input. No word or symbol allowed. Please enter a number."),
        }
    }
}

/// Main menu features
mod menu {
    use crate::{get_amount_as_float, get_input, Bill, Bills};

    /// Acceps mutable reference to the 'Bills' struct in order to add new bills to the struct
    pub fn add_bill(bills: &mut Bills) {
        println!("Bill name:");

        // Name
        let name = match get_input() {
            // Populate 'name' with the input, otherwise get out of the function
            Some(input) => input,
            None => return,
        };

        // Amount
        let amount = match get_amount_as_float() {
            Some(amount) => amount,
            None => return,
        };

        // Create the bill
        // Field names are the same as the variable names, no need to do assignments, i.e.: 'name: name'
        let bill = Bill { name, amount };
        bills.add(bill);
        println!("Bill added successfully!");
    }

    /// View all bills
    pub fn view_bills(bills: &Bills) {
        for bill in bills.get_all_bills() {
            // Same as 'println!("{:?}", bill);'
            println!("{bill:?}");
        }
    }
}

/// Main menu options
enum MainMenu {
    AddBill,
    ViewBill,
}

/// Determine choice by implementing the 'MainMenu' options
impl MainMenu {
    pub fn get_menu_string(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(MainMenu::AddBill),
            "2" => Some(MainMenu::ViewBill),
            _ => None,
        }
    }

    fn show() {
        println!("");
        println!(" == Billy The Bill Manager == ");
        println!("1. Add Bill");
        println!("2. View Bill");
        println!("");
        println!("Enter selection: ");
    }
}

fn main() {
    // Create bill structure
    let mut build_bill_struct = Bills::new();

    loop {
        // Display the menu
        MainMenu::show();

        // Get input
        let input = get_input().expect("no data entered");

        // 'match' the user input to determine which feature to execute
        // 'as_str()' to transform 'String' into '&str'
        match MainMenu::get_menu_string(input.as_str()) {
            Some(MainMenu::AddBill) => menu::add_bill(&mut build_bill_struct),
            Some(MainMenu::ViewBill) => menu::view_bills(&build_bill_struct),
            // If 'None' is returned, exit the program
            None => return,
        }
    }
}
