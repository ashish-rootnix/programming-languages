/*
    Error messages

*/

fn main()
{
    // Example of a syntax error
    // let x = 5 // Missing semicolon

    // Example of a type error
    // let y: i32 = "Hello"; // Cannot assign a string to an integer variable

    // Example of an unused variable warning
    let _unused_variable = 42; // Prefixing with an underscore suppresses the unused variable warning

    // Example of a dead code warning
    #[allow(dead_code)] // This attribute allows unused code without generating a warning
    fn unused_function() {
        println!("This function is not used, but it won't generate a warning.");
    }
}