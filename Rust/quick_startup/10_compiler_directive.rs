/*
    Compiler Directives
        - Attributes
*/
#[allow(dead_code)] // This attribute allows unused code without generating a warning
fn unused_function() {
    println!("This function is not used, but it won't generate a warning.");
}

fn square(x: i32) -> i32
{
    x * x
}
fn main() {
    
    let result = square(5);
    println!("The square of 5 is: {result}");

    #[allow(unused_variables)]
    let i: i32 = 10;

    #[allow(unused_variables)]
    let s: String = String::from("Hello, world!"); // Prefixing with an underscore also suppresses the unused variable warning
}
