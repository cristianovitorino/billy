use colored::*;

/// 'Update Bill' submenu
pub fn show_update_options() {
    println!("");
    println!("Bill found! What do you want to change?");
    println!("1. Name");
    println!("2. Value");
    println!("");
    println!("Enter selection: ");
}

/// Main menu options
pub enum MainMenu {
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
        // Ref: https://www.w3.org/TR/xml-entity-names/025.html
        println!("");
        println!("┌────────────────────────┐");
        println!("{}", "│ Billy The Bill Manager │".bold());
        println!("└────────────────────────┘");
        println!("");
        println!("1. Add Bill");
        println!("2. View Bill");
        println!("3. Remove Bill");
        println!("4. Update Bill");
        println!("5. Bill Total");
        println!("");
        println!(
            "{}",
            "You can press 'Enter' at any point to get back to this menu"
                .italic()
                .dimmed()
        );
        println!("");
        println!("Enter selection: ");
    }
}
