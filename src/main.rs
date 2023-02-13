use billy::{bills::*, input::*, menu::*, options::*};
// TODO  Refactor code where needed, if needed
// TODO  Add a persistent database

// Ref 1: https://doc.rust-lang.org/edition-guide/rust-2018/path-changes.html
// Ref 2: https://stackoverflow.com/a/26390046

fn run() -> Option<()> {
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
            Some(MainMenu::AddBill) => add_bill(&mut build_bill_struct),
            Some(MainMenu::ViewBill) => view_bills(&build_bill_struct),
            Some(MainMenu::RemoveBill) => remove_bill(&mut build_bill_struct),
            Some(MainMenu::UpdateBill) => update_bill(&mut build_bill_struct),
            Some(MainMenu::BillTotal) => bill_total(&mut build_bill_struct),
            // If 'None' is returned, exit the program
            None => break,
        }
    }
    None
}

fn main() {
    run();
}
