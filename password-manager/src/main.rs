// import input/output from the standard library.
use std::io;

// Enable comparing variables.
use std::cmp::Ordering;

// Enable working with CSV's.
use csv::ReaderBuilder;

// Create a struct for the Password Manager.
struct Manager {
    master_password: String,
    active: bool,
}

// Create methods for Manager.
impl Manager{
    fn set_master_password(&mut self) {
        println!("Please enter your master password: ");
        let mut password_input = String::new();
        io::stdin()
            .read_line(&mut password_input)
            .expect("Failed to read line!");
        self.master_password = password_input;
    }
    
    fn get_permission(&self) {
        println!("Please enter your master password: ");
        let mut password_input = String::new();
        io::stdin()
            .read_line(&mut password_input)
            .expect("Failed to read line!");
    }
}
fn main() {
    println!("Hello, world!");
}

