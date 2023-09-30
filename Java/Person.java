import java.util.Arrays;
import java.util.Calendar;

/*Class for a person's profile, including information 
about them and connections to others on the platform*/
public class Person {
    String name = "Bob", birthdate;
    int age;
    Person[] friendsList = {};


    //Constructor
    public Person(String name, int age) {
        this.name = name;
        this.age = age;
        Calendar c1 = Calendar.getInstance();
    }

    //Adds someone to a person's friend list
    void addFriend(Person friend) {
        Person[] newList = Arrays.copyOf(this.friendsList, this.friendsList.length + 1, Person[].class);
        newList[newList.length] = friend;
        this.friendsList = newList;
    }
}
