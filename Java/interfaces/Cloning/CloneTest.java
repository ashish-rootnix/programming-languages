package interfaces.Cloning;

public class CloneTest {
    public static void main(String[] args) throws CloneNotSupportedException
    {
        var original = new Employee("John", 50000);
        original.setHireDay(2025, 1, 1);

        Employee copy = original.clone();
        copy.raiseSalary(10);
        copy.setHireDay(2026, 1, 1);
        System.out.println("Orignal="+original);
        System.out.println("Copy="+copy);        
    }
}
