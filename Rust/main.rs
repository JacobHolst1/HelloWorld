// use dict::{Dict, DictIface};

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
        "3" => { println!("You're password today is: schlindikrop15");
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

    // Make a new password to replace it
}

fn generate_password(){

}