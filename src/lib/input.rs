use std::io;

/// Get the user input data as a 'String'
pub fn get_input() -> Option<String> {
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

/// Get user input after the 'Update Bill' main menu option is chosen
pub fn get_update_input() -> Option<String> {
    println!("Name:");
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Error, please enter your data again");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

/// Transform the 'String' bill amount input into a 'f64'
pub fn get_amount_as_float() -> Option<f64> {
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

        // Parse the input 'String' into a ´f64'
        // Let Rust figure out the error type since it's not relevant
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Invalid input. No word or symbol allowed. Please enter a number."),
        }
    }
}
