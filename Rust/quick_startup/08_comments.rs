/*
    Coometing
        - Comments
    More on print

*/

fn main() {
    // This is a single line comment

    /*
        This is a multi-line comment
        It can span multiple lines
    */

    println!("Hello, world!"); // This is an inline comment

    print!("Hello "); // This will print without a newline
    print!("World!"); // This will continue on the same line

    // Escape sequences
    println!("This is a new line\nThis is on a new line");
    println!("This is a tab\tThis is after a tab");
    println!("This is a backslash \\");
    println!("This is a double quote \"This is in quotes\"");
    println!("This is a single quote \'This is in single quotes\'");
    println!("This is a carriage return\rThis will overwrite the line");
    
    //Positional arguments
    println!("The first argument is {0}, and the second argument is {1}", "first", "second");
    println!("The second argument is {1}, and the first argument is {0}", "first", "second");
    //Named arguments
    println!("The name is {name}, and the age is {age}", name="Alice", age=30);
    
}