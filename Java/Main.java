import java.util.Scanner;

public class Main {
    //Var for current person using the application
    static Person currentUser;

    public static void main(String[] args) {
        System.out.println("Hello World");
        Scanner scanner = new Scanner(System.in);
        Data database = new Data();

        //First login screen
        Print("Welcome! Please sign in below");
        Print("or type 'new' to create a new account.");
        String userName = scanner.nextLine();
        if (userName == "new") {
            Print("Please enter your name, birthdate(formatted dd/MM/yyyy), and email address.");
            String name = scanner.nextLine();
            String birthdate = scanner.nextLine();
            String email = scanner.nextLine();
            Person newPerson = new Person(name, birthdate, email);
            currentUser = newPerson;
        } else {
            currentUser = database.getUserData("Jim");
        }

        //Once logged in, main menu appears
        boolean isGoing = true;
        while (isGoing) {
            Print("What would you like to do today?");
            Print("1. Make a post");
            Print("2. Edit profile");
            Print("3. Add friends");
            Print("4. Quit");

            switch (scanner.nextLine()) {
                case "1":
                    Print("What's on your mind today?");
                    //currentUser.Post(post);
                    break;
                case "2":
                    Print("What would you like to change?");
                    Print("1. Name  2. Birthday  3.Email");
                    
                    switch (scanner.nextLine()) {
                        case "1":
                            currentUser.name = scanner.nextLine();
                            break;
                        case "2":
                            currentUser.birthdate = scanner.nextLine();
                            break;
                        case "3":
                            currentUser.email = scanner.nextLine();
                            break;
                    }
                    
                    break;
                case "3":
                    Print("3");
                    break;
                case "4":
                    Print("4");
                    isGoing = false;
            }
                
        }
        scanner.close();
        System.out.println("You have been successfully logged out.");
    }

    // Condense System.out.println to a method called "Print()"
    static void Print(String thing) {
        System.out.println(thing);
    }

    public void mainLoop() {
        
    }
    
    public void newProfile() {
        
    }

}
