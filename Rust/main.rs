// use dict::{Dict, DictIface};

fn main(){
    let mut name = String::new();
    println!("What's your name?");
    let b1 = std::io::stdin().read_line(&mut name).unwrap();
    println!("Welcome to the password generator, {}", name);

    // Send the user to the main menu
    menu()

}

fn menu(){
    // Print out the options
    /*1. Make new password
    2. Edit a password
    3. Generate a password*/
    let mut ans = String::new();
    println!("Please select an option below: ");
    let b1 = std::io::stdin().read_line(&mut ans).unwrap();
    // match {}
}

fn new_password(){
    // Get input from the user for the application/site/whatever linked with the password
    println!("Welcome to password generator!")

    // Input the password. If it isn't very secure, prompt the user if they want to keep it


}

fn edit_password(){
    // Which password do we want to edit?

    // Make a new password to replace it
}

fn generate_password(){

}