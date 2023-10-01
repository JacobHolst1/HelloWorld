import java.io.File;
import java.io.FileNotFoundException;
import java.util.Arrays;
import java.util.Scanner;

//Database class that stores the users of the program
public class Data {
    
    //The list of users
    String[] usernames = {};
    Person[] users = {};

    //Constructor
    Data() {
        users = getUserList("users");
    }

    // Save a new user's profile
    void saveUser() {
        try {
            File file = new File("textFile.txt");
            boolean value = file.createNewFile();
            if (value) {
                System.out.println("New file was created.");
            } else {
                System.out.println("The file already exists.");
            }
        } catch (Exception e) {
            e.getStackTrace();
        }

        return;
    }
    
    //Find a specific user in the list given their name
    Person getUserData(String name) {
        String[] data = {};
        try {
            File file = new File(name +".txt");
            Scanner scanner = new Scanner(file);
            while (scanner.hasNextLine()) {
                String[] newList = Arrays.copyOf(data, data.length + 1, String[].class);
                newList[newList.length] = scanner.nextLine();
                data = newList;
                //debug print
                System.out.println(data);
        }
        scanner.close();
        } catch (FileNotFoundException e) {
            System.out.println("Error: file not found.");
            e.printStackTrace();
        }
        Person person = new Person(data[0], data[1], data[2]);
        return person;
    }
    
    //Retrieve the full list of users stored on 'users.txt'
    Person[] getUserList(String fileName) {
        Person[] users = {};
        try{
            File file = new File("users.txt");
            Scanner scanner = new Scanner(file);
            while (scanner.hasNextLine()){
                Person person = this.getUserData(scanner.nextLine());
                Person[] newList = Arrays.copyOf(users, users.length + 1, Person[].class);
                // newList[newList.length] = scanner.nextLine();
                newList[newList.length] = person;
                users = newList;
                scanner.close();
            }
        }catch (FileNotFoundException e) {
            System.out.println("Error: file not found.");
            e.printStackTrace();
        }
        // Person person = new Person("John Doe", "17", "johndoe@gmail.com");
        // Person[] users = { person, person };
        return users;
    }
}
