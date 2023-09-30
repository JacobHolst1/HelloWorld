import java.io.File;

//Database class that stores the users of the program
public class Data {
    
    //The list of users
    Person[] users = {};

    //Constructor
    Data() {
        users = getUserList();
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
    
    //Find a specific user in the list given a name
    void getUser(String name) {

    }
    
    //Retrieve the list of users
    Person[] getUserList() {

        Person person = new Person("John Doe", 17);
        Person[] users = { person, person };
        return users;
    }
}
