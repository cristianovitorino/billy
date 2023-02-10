use std::collections::HashMap;
use std::io;

/// The bills template
/// 'Debug' to easily print on terminal and 'Clone' to make copies of the struct
#[derive(Debug, Clone)]
pub struct Bills {
    info: HashMap<String, f64>,
}

/// 'Bills' "storage" superstructre implementation
impl Bills {
    /// Instanciate a new 'Bills' struct with an empty HashMap
    fn new() -> Self {
        Self {
            info: HashMap::new(),
        }
    }

    /// Add a new bill
    /// '&mut self' access mutably the 'info: HashMap<String, f64>' on the 'Bills' struct
    /// The owned 'name' and 'amount' moves into the 'Bills' struct type 'HashMap<String, f64>'
    /// Inserts the new bill 'info' into the HashMap
    fn add(&mut self, name: String, amount: f64) {
        self.info.insert(name, amount);
    }

    /// Print the bills
    /// Reference itself '&self' so it's able to access the 'Bills' struct
    /// Return a reference to the 'Bill' struct info 'HashMap<String, f64>'
    fn get_all_bills(&self) -> &HashMap<String, f64> {
        return &self.info;
    }

    /// Remove a bill
    fn remove(&mut self, name: String) {
        self.info.remove(&name);
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
    use crate::{get_amount_as_float, get_input, Bills};

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
        bills.add(name, amount);
        println!("Bill added successfully!");
    }

    /// View all bills
    pub fn view_bills(bills: &Bills) {
        for bill in bills.get_all_bills() {
            // Same as 'println!("{:?}", bill);'
            println!("{bill:?}");
        }
    }

    /// Remove a bill
    pub fn remove_bill(bills: &mut Bills) {
        println!("Bill name:");

        // Name
        let name = match get_input() {
            // Populate 'name' with the input, otherwise get out of the function
            Some(input) => input,
            None => return,
        };

        // Remove the bill
        bills.remove(name);
        println!("Bill removed successfully!");
    }
}

/// Main menu options
enum MainMenu {
    AddBill,
    ViewBill,
    RemoveBill,
}

/// Determine choice by implementing the 'MainMenu' options
impl MainMenu {
    pub fn get_menu_string(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(MainMenu::AddBill),
            "2" => Some(MainMenu::ViewBill),
            "3" => Some(MainMenu::RemoveBill),
            _ => None,
        }
    }

    fn show() {
        println!("");
        println!(" == Billy The Bill Manager == ");
        println!("1. Add Bill");
        println!("2. View Bill");
        println!("3. Remove Bill");
        println!("");
        println!("Enter selection: ");
    }
}

fn main() {
    // Create the bill structure
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
            Some(MainMenu::RemoveBill) => menu::remove_bill(&mut build_bill_struct),
            // If 'None' is returned, exit the program
            None => return,
        }
    }
}
