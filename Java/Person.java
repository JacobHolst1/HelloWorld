import java.util.Arrays;
import java.util.Calendar;

/*Class for a person's profile, including information 
about them and connections to others on the platform*/
public class Person {
    String name, birthdate, email;
    int age;
    Person[] friendsList = {};


    //Constructor
    public Person(String name, String birthdate, String email) {
        this.name = name;
        //TODO convert string to datetime object, then set age based on date
        this.birthdate = birthdate;
        // this.age = birthdate.year;
        // Calendar c1 = Calendar.getInstance();
        this.email = email;
    }

    //Adds someone to a person's friend list
    void addFriend(Person friend) {
        Person[] newList = Arrays.copyOf(this.friendsList, this.friendsList.length + 1, Person[].class);
        newList[newList.length] = friend;
        this.friendsList = newList;
    }
}
