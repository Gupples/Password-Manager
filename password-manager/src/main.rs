use core::panic;
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
    

    pub fn quit(&mut self) {
        self.active = false;
    }



    fn set_master_password(&mut self) {
        let mut password_check = get_input(
            "Please enter your master password: ".to_string());
        
        if password_check == self.master_password {
            let mut new_password1 = get_input(
                "Please enter your new master password: ".to_string());
            let mut new_password2 = get_input(
                "Please enter your new master password again: ".to_string());
            if new_password1 == new_password2{
                self.master_password = new_password1;
                println!("Password successfully changed!");
            } else {
                println!("Your passwords did not match.");
            }

        } else {
            println!("Incorrect master password.");
        }
        
    }

}
fn main() {
    let mut password_manager = Manager{
        master_password: String::from(""),
        active: true,
        sites: [].to_vec(),
        usernames: [].to_vec(),
        passwords: [].to_vec()
        
    };

    if get_input("Master Password: ".to_string())
     != password_manager.master_password {
         println!("Incorrect master password. ");
         
    } else {
        while password_manager.active{
                    let actions : Vec<&str> = [
            // "1. View entries,",
            // "2. Add entry,",
            // "3. Edit entry,",
            // "4. Delete entry,",
            "5. Change Master Password",
            "0. Quit,"].to_vec();
            for item in actions {
                println!("{item}");
            }
            let action = get_number_input("Choose an action: ".to_string());
            match action {
                0 => password_manager.quit(),
                5 => password_manager.set_master_password(),
                _ => panic!()
                
            }
        }
    }
}

fn get_input(prompt: String) -> String {
        println!("{prompt}");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        return input;
    }

    fn get_number_input(prompt: String) -> u32 {
        let mut is_number: bool = false;
        let mut number = 0;
        while !is_number {
            let number = get_input(prompt.clone());
            let number: u32 = match number.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
            };
        }
        return number;
    }