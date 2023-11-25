// use dict::{Dict, DictIface};
// use std::convert::From;

fn main(){
    let mut name = String::new();
    println!("What's your name?");
    let _ = std::io::stdin().read_line(&mut name).unwrap();
    println!("Welcome to the password generator, {}", name);

    // Send the user to the main menu, keeping them there until they select quit
    while menu() == true{
        println!("-----------------------------")
    }

}

fn menu() -> bool{

    // Print out the options
    let mut ans = String::new();
    println!("Please select an option below: 
    [1] Make a new password
    [2] Edit a password
    [3] Generate a password
    [4] Quit
    ");

    // Process the input
    let _ = std::io::stdin().read_line(&mut ans).unwrap();
    match ans.trim_end(){
        "1" => { println!("What will the password be?");
                    new_password();
                    return true;
                },
        "2" => { println!("Which account would you like to change the password for?");
                    edit_password();
                    return true;
                },
        "3" => { println!("Password length?");
                    generate_password();
                    return true;
                },
        _ => {println!("Byyyyyyye!");
                return false;
            }
    }
}

fn new_password(){
    // Get input from the user for the application/site/whatever linked with the password
    println!("Welcome to password generator!")

    // Input the password. If it isn't very secure, prompt the user if they want to keep it


}

fn edit_password(){
    // Which password do we want to edit?
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input).unwrap();

    // Make a new password to replace it
    println!("What will the new password be?");
    let mut input2 = String::new();
    let _ = std::io::stdin().read_line(&mut input2).unwrap();

    // Print result
    println!("The new password for {0} is {1}", input, input2);
}

fn generate_password(){
    // Prompt for length of password to generate
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input).unwrap();
    let intput: i32 = input.trim_end().parse().unwrap();
    
    // Generate
    for _ in 1..=intput{
        print!("a");
    }
    println!("Success!");

    // Print the password and prompt for connecting it to an account


}