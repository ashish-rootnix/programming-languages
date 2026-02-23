/*
    Borrowing:
        - In Rust, borrowing is a way to access data without taking ownership of it. 
          This allows you to use values in a function without transferring ownership, and the original owner retains control over the value.
        - When you borrow a value, you create a reference to it. 
          The reference can be either immutable (read-only) or mutable (read-write), depending on how you declare it.
        - Rust's borrowing rules ensure that references are always valid and that there are no data races or other unsafe behaviors when accessing shared data.
        - When you borrow a value, you can use it within the scope of the borrow, but you cannot modify it unless you have a mutable reference. 
          This helps prevent bugs and ensures that data is accessed safely and predictably.
        - Borrowing is a fundamental concept in Rust that allows for safe and efficient memory management without the need for a garbage collector, while still enabling powerful programming
            patterns and abstractions.
    
    Data Structures:
        - Rust provides several built-in data structures that are commonly used in programming. These include:
            - &str and String: &str is a string slice that represents a view into a string, while String is a growable, heap-allocated data structure used to store and manipulate text.
            - Arrays: A fixed-size collection of elements of the same type, determined at compile time and stored on the stack.
            - Vectors: A growable, heap-allocated collection of elements of the same type that can change in size at runtime.
            - Tuples: A fixed-size collection of elements of different types that can hold a heterogeneous set of values.
            - Empty Tuple: A special type that represents the absence of a value, often used as the return type of functions that do not return 
            any meaningful value or as a placeholder when a value is required but not needed.
            - These data structures are designed to work seamlessly with Rust's ownership and borrowing system, allowing for safe and 
            efficient memory management while still providing powerful abstractions for working with data.

*/

fn main()
{
    let mut vec1: Vec<i32> = vec![1, 2, 3, 4, 5]; // vec4 is the owner of the vector [1, 2, 3, 4, 5] 
    let _ref1: &Vec<i32> = &vec1; // ref1 is an immutable reference to vec1, allowing us to read the vector without modifying it
    let _ref2 = &vec1; // ref2 is an immutable reference to vec1, allowing us to read the vector without modifying it
    println!("vec1: {vec1:?}, ref1: {_ref1:?}, ref2: {_ref2:?}"); 
    // Printing the vector and the references to show that they all point to the same underlying data

    let ref3: &mut Vec<i32> = &mut vec1; // ref3 is a mutable reference to vec1, allowing us to modify the vector through ref3
    ref3.push(7); // Modifying the vector through the mutable reference ref3 
    
    // At a time either we can have one mutable reference or any number of immutable references, but not both at the same time.
    // This is because mutable references allow for modification of the data, and having immutable references at the same time
    // could lead to data   races and other unsafe behaviors.

    // Error: cannot borrow `vec1` as mutable because it is also borrowed as immutable
    //println!("vec1: {vec1:?}, ref1: {ref1:?}, ref2: {ref2:?}, ref3: {ref3:?}"); 
    // Printing the vector and the references to show that they all point to the same underlying data and reflect the modifications made through the mutable references.

    println!(" vec1: not allowed to access directly, ref1: out of scope, ref2: out of scope, ref3: {ref3:?}");
}
