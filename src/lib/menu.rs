use super::{display::*, input::*, structure::*};
use colored::*;

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
    println!("");
    println!("{}", "Bill added successfully".italic().dimmed());
}

/// View all bills
pub fn view_bills(bills: &Bills) {
    if bills.info.is_empty() {
        println!("");
        println!("{}", "There is no bill data to display".italic().dimmed());
    } else {
        println!("{0: <12} │ {1: <12}", "NAME".bold(), "AMOUNT".bold());
        for (k, v) in bills.get_all_bills() {
            // Ref 1: https://stackoverflow.com/a/69981450
            // Ref 2: https://crates.io/crates/colored

            println!("{0: <12}{1: <12}", "─────────────┼", "──────────────");
            println!("{0: <12} │ {1: <12}", k.italic(), v)
        }
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
            println!("Bill removed successfully");
            bills.remove(name);
            break;
        } else {
            println!("");
            println!(
                "{}",
                "No bill found with this name, try again".italic().dimmed()
            );
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
        // Check if data exists with that name and perform appropriate actions
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
                println!("");
                println!("{}", "Bill name updated successfully".italic().dimmed());
            } else if choice == "2" {
                let new_amount = match get_amount_as_float() {
                    Some(amount) => amount,
                    None => return,
                };
                bills.update_value(old_name, new_amount);
                println!("");
                println!("{}", "Bill value updated successfully".italic().dimmed());
            } else {
                println!("");
                println!("{}", "Invalid choice. Try again".italic().dimmed());
            }
            break;
        } else {
            println!("");
            println!(
                "{}",
                "No bill found with this name, try again".italic().dimmed()
            );
        }
    }
}
pub fn bill_total(bills: &mut Bills) {
    let mut total_value = 0.0;
    if bills.info.is_empty() {
        println!("");
        println!("{}", "There is no bill data to display".italic().dimmed());
    } else {
        for (_k, v) in bills.info.iter() {
            total_value += v;
        }
        println!("");
        println!("{} {}", "Bill total:".bold(), total_value);
    }
}
