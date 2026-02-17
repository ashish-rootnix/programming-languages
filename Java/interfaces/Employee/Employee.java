package interfaces.Employee;

public class Employee implements Comparable<Employee>
{    
    private String name;
    private double salary;

    public Employee(String name, double salary)
    {
        this.name = name;
        this.salary = salary;
    }

    public String getName()
    {
        return name;
    }

    public double getSalary()
    {
        return salary;
    }

    public void raiseSalary(double byPercent)
    {
        double raise = salary * byPercent / 100;
        salary += raise;
    }

    /*Compares employee by salary
    * @param otehr another Employee object
    * @return a negative value if employee has a lower salary than other, a positive value otherwise
    * 0 if the salaries are the same.
    */
    public int compareTo(Employee other)
    {
        return Double.compare(salary, other.salary);
    }
}
