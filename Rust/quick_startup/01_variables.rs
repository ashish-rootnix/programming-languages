/*
    Varaibles:
        Definitions
        Mutability
        Scope
        Shadowing
    Constants:
        Definitions
        Differences from Varaibles
*/

fn main()
{
    // Variable Definition
    // In Rust, you can declare a variable using the `let` keyword followed by the variable name and an optional type annotation.
    // The type annotation is not always necessary, as Rust can often infer the type based on the value assigned to the variable.
    let x:i16 = 10; 
    println!("x is: {x}");

    // Mutability - Mutability allows you to modify value after you have declared it. 
    // By default, variables in Rust are immutable, meaning their values cannot be changed once assigned. 
    // To make a variable mutable, you need to use the `mut` keyword when declaring it.
    let mut y:i32 = 20;
    println!("y is: {y}");
    y = 30;
    println!("Mutated y is: {y}");

    let mut p1 = 40;
    let p2;
    p1 = p1 * 3; 
    p2 = p1 - 2; // In Rust, immutable variables can be assigned a value once after declaration, as long as they are not modified afterward
    //p2 = 100; // Error: cannot assign twice to immutable variable `p2`
    println!("p1 is: {p1}, p2 is: {p2}");

    //Scope - Scope refers to the region of the code where a variable is valid and can be accessed.
    // In Rust, variables have a specific scope, which is determined by the block of code in which they are declared.
    {
        let z:i64 = 40;
        println!("z is: {z}");
    }
    //let s = z; // the binding `z` is available in a different scope in the same function

    // Shadowing - Shadowing allows you to declare a new variable with the same name as a previous variable. 
    // The new variable shadows the previous one, meaning that the previous variable is no longer accessible. 
    // This can be useful when you want to reuse a variable name but with a different type or value.
    let t:i32 = 10;
    let t:i32 = t + 10; // Shadowing allows us to reuse the same variable name
    println!("t is: {t}");

    let _u: i32 = 3;
    let _u: f64 = 3.0; // Shadowing allows us to change the type of the variable
    println!("_u is: {_u}");

    let v:i32 = 30;
    {
        let v:i32 = 40; // Shadowing allows us to reuse the same variable name in a different scope
        println!("v in inner scope is: {v}");
    }
    println!("v in outer scope is: {v}");
    
    // Constants: Constnt must initlized at compile time and cannot be changed at runtime. 
    // They are typically used for values that are known at compile time and will not change throughout the program.
    const MAX_VALUE: u32 = 100; // Constants are declared using the `const` keyword and must have a type annotation.
    println!("The maximum value is: {MAX_VALUE}");
}