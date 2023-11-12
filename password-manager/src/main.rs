// import input/output from the standard library.
use std::io;

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

    fn quit(&mut self) {
        self.active = false;
    }

    fn get_master_password(&self) -> &str {
        &self.master_password.as_str()
    }

    // 1. View entries.
    fn view_entries(&self) {
        if self.sites.len() != 0{
            for n in 0..self.sites.len() {

                print!("{}\tURL: {}\tUsername: {}\tPassword: {}", n + 1,
                 self.sites[n], self.usernames[n], self.passwords[n]);
            }
        } else {
            print!("There are no entries in your password manager. ");
            println!("Please add some first.");
        }
    }

    // 2. Add entry.
    fn add_entry(&mut self) {
        let site = get_input("URL: ".trim().to_string());
        let username = get_input("Username: ".trim().to_string());
        let password = get_input("Password: ".trim().to_string());
        self.sites.push(site);
        self.usernames.push(username);
        self.passwords.push(password);
        
    }
    
    // 3. Edit entry.
    fn edit_entry(&mut self){
        
        // Display entries.
        self.view_entries();

        // Get entry to edit.
        let edit = get_number_input("Which entry would you like to edit? "
            .to_string()) - 1;
        
        // Edit values in vectors.
        self.sites[edit as usize] = get_input("URL: ".to_string());
        self.usernames[edit as usize] = get_input("Username: ".to_string());
        self.passwords[edit as usize] = get_input("Password: ".to_string());
        println!("Entry successfully edited.");
        
        
    }
    
    // 4. Delete entry.
    fn delete_entry(&mut self){
        
        // Display entries.
        self.view_entries();
        
        // Get entry to delete.
        let delete = get_number_input("Which entry would you like to delete? "
            .to_string());
        
        // Delete those values in vectors.
        self.sites[delete as usize].pop();
        self.usernames[delete as usize].pop();
        self.passwords[delete as usize].pop();
        println!("Entry successfully deleted.");

    }

    // 5. Change master password.
    fn change_master_password(&mut self) {
        let password_check = get_input(
            "Please enter your master password: ".to_string());
        
        if password_check == self.master_password {
            let new_password1 = get_input(
                "Please enter your new master password: ".to_string());
            let new_password2 = get_input(
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

fn get_input(prompt: String) -> String {
    println!("{prompt}");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    return input;
}

fn get_number_input(prompt: String) -> u32 {
    let number = get_input(prompt.clone());
    let number: u32 = match number.trim().parse(){
        Ok(num) => num,
        Err(_) => 0,
    };
    return number;
}
fn main() {
    let mut password_manager = Manager{
        master_password: get_input("Enter a starting master password: "
            .to_string()),
        active: true,
        sites: [].to_vec(),
        usernames: [].to_vec(),
        passwords: [].to_vec()
        
    };
    if get_input("Enter your master password: ".to_string())
     != password_manager.get_master_password() {
         println!("Incorrect master password. ");
         
    } else {
        while password_manager.active{
            let actions : Vec<&str> = [
            "\n1. View entries,",
            "2. Add entry,",
            "3. Edit entry,",
            "4. Delete entry,",
            "5. Change Master Password",
            "0. Quit\n"].to_vec();
            for item in actions {
                println!("{item}");
            }
            let action = get_number_input("Choose an action: ".to_string());
            println!("");
            match action {
                0 => password_manager.quit(),
                1 => password_manager.view_entries(),
                2 => password_manager.add_entry(),
                3 => password_manager.edit_entry(),
                4 => password_manager.delete_entry(),
                5 => password_manager.change_master_password(),
                _ => println!("That wasn't an option")
                
            }
        }
    }
}