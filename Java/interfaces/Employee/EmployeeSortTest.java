package interfaces.Employee;

import java.util.*;

public class EmployeeSortTest {
    public static void main(String[] args)
    {
        var staff = new Employee[3];

        staff[0] = new Employee("Harry Hacker", 35000);
        staff[1] = new Employee("Sakshi Shelke", 100000);
        staff[2] = new Employee("Mayur Charkha", 100000);

        Arrays.sort(staff);

        // Print out information about all Employee objects
        for(Employee e : staff)
        {
            System.out.println("name= " + e.getName() + " ,salary: " + e.getSalary());
        }
    }
}
