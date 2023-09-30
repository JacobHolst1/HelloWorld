import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        System.out.println("Hello World");
    }

    static void Print(String thing) {
        // Condense System.out.println to a method called "Print()"
        System.out.println(thing);
    }

    public void mainLoop() {
        Scanner scanner = new Scanner(System.in);
        Data database = new Data();
        boolean isGoing = true;
        while (isGoing) {
            Print("Welcome!Please sign in below");
            Print("or type 'new' to create a new account.");
            String userName = scanner.nextLine();
            if (userName == "new") {
                Print("Please enter your name, birthdate(formatted dd/MM/yyyy), and email address.");
                String name = scanner.nextLine();
                //TODO convert string to datetime object
                String birthdate = scanner.nextLine();
                String email = scanner.nextLine();
                

            } else {
                isGoing = false;
            }
            
            
        }
        scanner.close();
        System.out.println("You have been successfully logged out.");
    }
    
    public void newProfile() {
        
    }

}
