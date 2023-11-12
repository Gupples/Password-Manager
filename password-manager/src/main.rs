// import input/output from the standard library.
use std::io;

// Enable comparing variables.
use std::cmp::Ordering;

// Enable working with CSV's.
// use csv::ReaderBuilder;

// Create a struct for the Password Manager.
struct Manager {
    master_password: String,
    active: bool,
    sites: Vec<String>,
    usernames: Vec<String>,
    passwords: Vec<String>


}

// Create methods for Manager.
impl Manager{
    fn get_input(&self, prompt: String) -> String {
        println!("{prompt}");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        return input;
    }

    fn set_master_password(&mut self) {
        let mut password_input = self.get_input(
            "Please enter your master password: ".to_string());
        self.master_password = password_input;
    }

    fn action(&self){

    }
}
fn main() {
    let password_manager = Manager{
        master_password: String::from(""),
        active: true,
        sites: [].to_vec(),
        usernames: [].to_vec(),
        passwords: [].to_vec()
        
    };
    while password_manager.active{
        password_manager.action();
    }
}

