use super::{structure::*, display::*, input::*};

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
