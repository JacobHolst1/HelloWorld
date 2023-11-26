// use dict::{Dict, DictIface};
// use std::convert::From;
use std::fs::File;
use std::io::prelude::*;
// use std::io::BufReader;
use rand::Rng;
const FILENAME: &str = "pass.txt";


fn load_passwords()->std::io::Result<()> {
    /*
    //create a dictionary of strings
    let mut dict = Dict::<String>::new();
    assert_eq!( dict.is_empty(), true );
    assert_eq!( dict.len(), 0 );
    */
    let mut file = File::open(FILENAME)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    assert_eq!(contents, "Hello, world!");
    Ok(())
}

//------------------------------------------------------------------------------------------------------------

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
        "1" => {
                    new_password();
                    return true;
                },
        "2" => { 
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

//---------------------------------------------------------------------------------------------------------------------

fn save_password() -> std::io::Result<()> {
    // Open the file to save to
    let mut file = File::create("pass.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}

//------------------------------------------------------------------------------------------------------------------------

fn new_password(){
    // Get input from the user for the application/site/whatever linked with the password
    println!("Enter the account name to link a password to");
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input).unwrap();

    // Input the password. v2.0: If it isn't very secure, prompt the user if they want to keep it
    println!("Enter the password");
    let mut input2 = String::new();
    let _ = std::io::stdin().read_line(&mut input2).unwrap();

    println!("Success! The password for {0} is {1}.", input, input2)

    // Save the password to the text file
    //
}

fn edit_password(){
    // Which password do we want to edit?
    println!("Which account would you like to change the password for?");
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input).unwrap();

    // Make a new password to replace it
    println!("What will the new password be?");
    let mut input2 = String::new();
    let _ = std::io::stdin().read_line(&mut input2).unwrap();

    // Print result
    println!("The new password for {0} is now {1}", input, input2);
}

fn generate_password(){
    // Prompt for length of password to generate
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input).unwrap();
    let intput: i32 = input.trim_end().parse().unwrap();
    
    // Set the complexity of the password
    let mut complexity = String::new();
    let _ = std::io::stdin().read_line(&mut complexity).unwrap();
    match complexity.trim_end(){
        "basic" => {let chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];},
        "two-cased" => {let chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];},
        "numerical" => {let chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];},
        _ => {let chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '.', ',', '?', '!', '@', '#', '$', '%', '^', '&', '*', ';', ':'];}
    }

    // Create an array of filler characters, then fill them in with random selections
    let mut password[char; intput] = ['a'; intput];
    for i in 1..=intput{
        let random_index = rand::thread_rng().gen_range(0..chars.len());
        let newchar = chars[random_index];
        print!(newchar);
        password[i] = newchar;
    }
    let password_str: String = password.iter().collect();
    println!("");
    
    // Prompt for connecting the generated password to an account
    println!("Success! Would you like to link it to an account? [y], [n]");
    let _ = std::io::stdin().read_line(&mut input).unwrap();
    match input.trim_end() {
        "y" => save_password(),
        _ => 
    }

}