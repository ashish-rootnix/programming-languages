/*
    Conditionals:
        If else
        if else ladder
        Match
*/

fn main()
{
    let num : i32 = 40;

    if num < 50
    {
        println!("num is less than 50");
    }
    else
    {
        println!("num is greater than or equal to 50");
    }

    let num2 : i32 = 20;
    if num < 50 && num2 < 30
    {
        println!("num is less than 50 and num2 is less than 30");
    }
    else if num < 50 && num2 >= 30
    {
        println!("num is less than 50 and num2 is greater than or equal to 30");
    }
    else if num >= 50 && num2 < 30
    {
        println!("num is greater than or equal to 50 and num2 is less than 30");
    }
    else
    {
        println!("num is greater than or equal to 50 and num2 is greater than or equal to 30");
    }

    let grade: char = if num >= 90
    {
        'A'
    }
    else if num >= 80
    {
        'B'
    }
    else if num >= 70
    {
        'C'
    }
    else if num >= 60
    {
        'D'
    }
    else
    {
        'F'
    };
    println!("Grade is: {grade}");

    let day = 3;
    match day
    {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day"), // The _ pattern is a catch-all that matches any value that hasn't been
        //  matched by the previous patterns. It is often used to handle cases that are not
    }

    let marks: i32 = 95;
    let mut _grade : char = 'F';
    match marks{
        90..=100 => _grade = 'A',
        80..=89 => _grade = 'B',
        70..=79 => _grade = 'C',
        60..=69 => _grade = 'D',
        _ => _grade = 'F',
    }
    println!("Grade is: {_grade}");
}
