use std::collections::HashMap;
use std::io;

// TODO  Refactor code where needed, if needed
// TODO  Modularize the program
// TODO  Add a persistent database

/// The bills template
/// - 'Debug' to easily print on terminal and 'Clone' to make copies of the struct
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
    /// - '&mut self' access mutably the 'info: HashMap<String, f64>' on the 'Bills' struct
    /// - The owned 'name' and 'amount' moves into the 'Bills' struct type 'HashMap<String, f64>'
    /// - Inserts the new bill 'info' into the HashMap
    fn add(&mut self, name: String, amount: f64) {
        self.info.insert(name, amount);
    }

    /// Print the bills
    /// - Reference itself '&self' so it's able to access the 'Bills' struct
    /// - Return a reference to the 'Bill' struct info 'HashMap<String, f64>'
    fn get_all_bills(&self) -> &HashMap<String, f64> {
        return &self.info;
    }

    /// Remove a bill
    fn remove(&mut self, name: String) {
        self.info.remove(&name);
    }

    /// Remove the old key, keep the value
    fn update_key(&mut self, old_name: String, new_name: String) {
        // Ref: https://stackoverflow.com/a/64997032

        // - Get the old value from 'old_name'
        // - 'unwrap()' to get the data out of the 'Option'
        // - 'clone()' to take ownership
        let old_value = self.info.get(&old_name).unwrap();
        let old_value = old_value.clone();

        // Remove the old key
        self.info.remove(&old_name);

        // Insert a new key, keep the old value (provides a new value only if it doesn't already exist)
        self.info.entry(new_name).or_insert_with(|| old_value);
    }

    /// Update value only
    fn update_value(&mut self, name: String, amount: f64) {
        // Ref 1: https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html
        // Ref 2: https://stackoverflow.com/a/71185788
        // Ref 3: https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.and_modify
        // Ref 4: https://www.reddit.com/r/rust/comments/s2vu1u/how_to_update_hashmap/

        // Provides in-place mutable access to an occupied entry before any potential inserts into the map
        self.info.entry(name).and_modify(|value| *value = amount);
    }
}

/// Get the user input data as a 'String'
fn get_input() -> Option<String> {
    // Empty 'String' template
    let mut buffer = String::new();

    // Loop until gets valid data
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again.");
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

fn get_update_input() -> Option<String> {
    println!("Name:");
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again.");
    }
    let input = buffer.trim().to_owned();
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

fn show_update_options() {
    println!("");
    println!("Bill found! What do you want to change?");
    println!("1. Name");
    println!("2. Value");
    println!("");
    println!("Enter selection: ");
}

/// Main menu features
mod menu {
    use crate::{get_amount_as_float, get_input, get_update_input, show_update_options, Bills};

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
        // - Field names are the same as the variable names, no need to do assignments, i.e.: 'name: name'
        bills.add(name, amount);
        println!("Bill added successfully!");
    }

    /// View all bills
    pub fn view_bills(bills: &Bills) {
        for bill in bills.get_all_bills() {
            // Same as 'println!("{:?}", bill);'
            println!("{bill:?}");
            // TODO  Add feature to check if the hash is empty
            // TODO  Make a "pretty printer"
        }
    }

    /// Remove a bill
    pub fn remove_bill(bills: &mut Bills) {
        loop {
            println!("Bill name:");
            let name = match get_input() {
                Some(input) => input,
                None => return,
            };
            // Check if data exists with that name
            if bills.info.contains_key(&name) {
                println!("Bill removed successfully!");
                bills.remove(name);
                break;
            } else {
                println!("No bill found with this name, try again.");
            }
        }
    }

    /// Update a bill
    /// - Allows to change either 'key' or 'value'
    pub fn update_bill(bills: &mut Bills) {
        loop {
            println!("Find bill:");
            let old_name = match get_input() {
                Some(input) => input,
                None => return,
            };
            // Check if data exists with that name and perform apropriate actions
            if bills.info.contains_key(&old_name) {
                show_update_options();
                let choice = match get_input() {
                    Some(input) => input,
                    None => return,
                };
                if choice == "1" {
                    let new_name = match get_update_input() {
                        Some(input) => input,
                        None => return,
                    };
                    bills.update_key(old_name.to_owned(), new_name);
                } else if choice == "2" {
                    let new_amount = match get_amount_as_float() {
                        Some(amount) => amount,
                        None => return,
                    };
                    bills.update_value(old_name, new_amount);
                } else {
                    println!("Invalid choice. Try again.");
                }
                break;
            } else {
                println!("No bill found with this name, try again.");
            }
        }
    }
    pub fn bill_total(bills: &mut Bills) {
        let mut total_value = 0.0;
        for (_k, v) in bills.info.iter() {
            if v > &0.0 {
                total_value += v;
            } else {
                println!("There are no values. Try entering some.");
            }
        }
        println!("Bill total: {:?}", total_value);
    }
}

/// Main menu options
enum MainMenu {
    AddBill,
    ViewBill,
    RemoveBill,
    UpdateBill,
    BillTotal,
}

/// Determine choice by implementing the 'MainMenu' options
impl MainMenu {
    pub fn get_menu_string(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(MainMenu::AddBill),
            "2" => Some(MainMenu::ViewBill),
            "3" => Some(MainMenu::RemoveBill),
            "4" => Some(MainMenu::UpdateBill),
            "5" => Some(MainMenu::BillTotal),
            _ => None,
        }
    }

    pub fn show() {
        println!("");
        println!(" == Billy The Bill Manager == ");
        println!("1. Add Bill");
        println!("2. View Bill");
        println!("3. Remove Bill");
        println!("4. Update Bill");
        println!("5. Bill Total");
        println!("");
        println!("Enter selection: ");
    }
}

fn run_program() -> Option<()> {
    // Create the bill structure
    let mut build_bill_struct = Bills::new();

    loop {
        // Display the menu
        MainMenu::show();

        // Get input
        let input = get_input()?;

        // - 'match' the user input to determine which feature to execute
        // - 'as_str()' to transform 'String' into '&str'
        match MainMenu::get_menu_string(input.as_str()) {
            Some(MainMenu::AddBill) => menu::add_bill(&mut build_bill_struct),
            Some(MainMenu::ViewBill) => menu::view_bills(&build_bill_struct),
            Some(MainMenu::RemoveBill) => menu::remove_bill(&mut build_bill_struct),
            Some(MainMenu::UpdateBill) => menu::update_bill(&mut build_bill_struct),
            Some(MainMenu::BillTotal) => menu::bill_total(&mut build_bill_struct),
            // If 'None' is returned, exit the program
            None => break,
        }
    }
    None
}

fn main() {
    run_program();
}
