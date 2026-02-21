/*
    Functions
    Code blocks
*/

// Type must be explictly defined in function parameters and return type.
fn my_fn(s: &str)  
{
    println!("Inside my_fn(): {s}");
}

fn multiplication(num1: i32, num2: i32) -> i32
{
    println!("Inside multiplication(): num1 = {num1}, num2 = {num2}");
    num1 * num2 // The last expression in a function is the return value, and it does not require a semicolon.
}

fn basic_math(num1: i32, num2: i32) -> (i32, i32, i32, i32)
{
    println!("Inside basic_math(): num1 = {num1}, num2 = {num2}");
    let sum = num1 + num2;
    let difference = num1 - num2;
    let product = num1 * num2;
    let quotient = num1 / num2;

    (sum, difference, product, quotient) // Returning a tuple containing all the results
}

// Expressions and Statements:
// In Rust, an expression is a piece of code that evaluates to a value, while a
// statement is a piece of code that performs an action but does not return a value.
// For example, a function call is an expression because it evaluates to a value (the return
// value of the function), while a variable declaration is a statement because it performs an action
// (declaring a variable) but does not return a value.

fn main()
{
    my_fn("This is my function");

    let str: &'static str = "Function call with a variable";

    my_fn(str);

    let result = multiplication(5, 10);
    println!("Multiplication result: {result}");

    let (sum, difference, product, quotient) = basic_math(20, 5);
    println!("Sum: {sum}, Difference: {difference}, Product: {product}, Quotient: {quotient}");

    //Code blocks
    {
        let x = 10;
        let y = 20;
        println!("Inside code block: x = {x}, y = {y}");
    }
    // The variables x and y are not accessible outside the code block
    //println!("Outside code block: x = {x}, y = {y}");

    let code_block = {
        let a = 5;
        let b = 10;
        a + b // The last expression in the code block is the value of the block
    };
    println!("Value of code block: {code_block}");
}
