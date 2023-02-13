use std::collections::HashMap;

/// The bills template
/// - 'Debug' to easily print on terminal and 'Clone' to make copies of the struct
#[derive(Debug, Clone)]
pub struct Bills {
    pub info: HashMap<String, f64>,
}

/// 'Bills' "storage" superstructre implementation
impl Bills {
    /// Instanciate a new 'Bills' struct with an empty HashMap
    pub fn new() -> Self {
        Self {
            info: HashMap::new(),
        }
    }

    /// Add a new bill
    /// - '&mut self' access mutably the 'info: HashMap<String, f64>' on the 'Bills' struct
    /// - The owned 'name' and 'amount' moves into the 'Bills' struct type 'HashMap<String, f64>'
    /// - Inserts the new bill 'info' into the HashMap
    pub fn add(&mut self, name: String, amount: f64) {
        self.info.insert(name, amount);
    }

    /// Print the bills
    /// - Reference itself '&self' so it's able to access the 'Bills' struct
    /// - Return a reference to the 'Bill' struct info 'HashMap<String, f64>'
    pub fn get_all_bills(&self) -> &HashMap<String, f64> {
        return &self.info;
    }

    /// Remove a bill
    pub fn remove(&mut self, name: String) {
        self.info.remove(&name);
    }

    /// Remove the old key, keep the value
    pub fn update_key(&mut self, old_name: String, new_name: String) {
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
    pub fn update_value(&mut self, name: String, amount: f64) {
        // Ref 1: https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html
        // Ref 2: https://stackoverflow.com/a/71185788
        // Ref 3: https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.and_modify
        // Ref 4: https://www.reddit.com/r/rust/comments/s2vu1u/how_to_update_hashmap/

        // Provides in-place mutable access to an occupied entry before any potential inserts into the map
        self.info.entry(name).and_modify(|value| *value = amount);
    }
}
