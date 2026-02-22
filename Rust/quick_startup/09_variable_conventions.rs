/*
    Variable Conventions
    Unused variables
    Statics
*/

fn main()
{
    // Unused variable
    let _unused_variable = 42; // Prefixing with an underscore suppresses the unused variable warning

    // Static variable
    static GREETING: &str = "Hello, world!";
    println!("{}", GREETING);

    const PI: f32 = 3.14;
    let a : f32 = 2.0 * PI;
    let b : f32 = 2.0 * PI;
    println!("a: {a}, b: {b}");

    let x: &str = GREETING; // Using the static variable
    println!("x: {x}");

}