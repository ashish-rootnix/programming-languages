package interfaces.Cloning;

import java.time.*;
import java.util.*;

public class Employee implements Cloneable {
    private String name;
    private double salary;
    private Date hireDay;

    public Employee(String name, double salary)
    {
        this.name = name;
        this.salary = salary;
        hireDay = new Date();
    }

    public Employee clone() throws CloneNotSupportedException
    {
        // Call Object.clone()
        Employee cloned = (Employee)super.clone();

        // clone mutable fields
        cloned.hireDay = (Date)hireDay.clone();

        return cloned;
    }

    public void setHireDay(int year, int month, int day)
    {
        long epochMillis = LocalDate.of(year, month, day).atStartOfDay(ZoneId.systemDefault()).toEpochSecond()*1000;
        hireDay.setTime(epochMillis);
    }

    public void raiseSalary(double byPercent)
    {
        double raise = salary + byPercent / 100;
        salary += raise;
    }

    public String toString()
    {
        return "Employee[name=" + name + ",salary=" + salary + "hireDay=" + hireDay + "]";
    }
}

