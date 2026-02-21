/*
    Compound data types:
        &str and string - String is a growable, heap-allocated data structure that is used to store and manipulate text. 
            It is part of the Rust standard library and provides various methods for working with strings. The str type, 
            on the other hand, is a string slice that represents a view into a string. 
            It is an immutable reference to a sequence of UTF-8 bytes and is commonly used for string literals and borrowed 
            string data.

        Arrays - An array is a fixed-size collection of elements of the same type. The size of an array is determined at compile time 
            and cannot be changed at runtime. Arrays are useful when you know the number of elements in advance and want to store them
            in a contiguous block of memory.
            
        Vectors - A vector is a growable, heap-allocated collection of elements of the same type. Unlike arrays, vectors can change 
            in size at runtime. They provide more flexibility than arrays and are commonly used when you need to store a variable 
            number of elements.
        
        Tuples - A tuple is a fixed-size collection of elements of different types. It can hold a heterogeneous set of values, and the
            types of the elements can be different. Tuples are useful for grouping related values together 
            and can be used to return multiple values from a function.

        Empty Tuple - The empty tuple, denoted as `()`, is a special type that represents the absence of a value. 
            It is often used as the return type of functions that do not return any meaningful value, or 
            as a placeholder when a value is required but not needed.
*/

fn main()
{
    // String and &str
    let sliced_str: &str = "Fixed length string slice"; // &str is a string slice that represents a view into a string. 
    // It is an immutable reference to a sequence of UTF-8 bytes and is commonly used for string literals and borrowed string data.

    let fixed_static_str: &'static str = "Fixed length static string slice"; // The 'static lifetime indicates that the string slice 
    // has a static lifetime, meaning it will live for the entire duration of the program.

    let mut growable_string: String = String::from("This string can grow"); // String is a growable, 
    // heap-allocated data structure that is used to store and manipulate text.

    growable_string.push_str("and change"); // The push_str method is used to append a string slice to the end of the String.
    println!("Sliced str: {sliced_str}, Fixed static str: {fixed_static_str}, Growable string: {growable_string}");

    // Arrays
    let fixed_array: [i32; 5] = [1, 2, 3, 4, 5]; // An array is a fixed-size collection of elements of
    // the same type. The size of an array is determined at compile time and cannot be changed at runtime.
    println!("Fixed array: {:?}", fixed_array); // The {:?} format specifier is used

    let mut flexible_array: [i32; 3] = [10, 20, 30];
    flexible_array[0] = 15; // You can modify the elements of an array
    println!("Flexible array: {:?}", flexible_array); // format specifier {:?} is used to print the array in a debug format

    // Vectors
    let vec_1:Vec<i32> = vec![1, 2, 3, 4, 5]; // A vector is a growable, heap-allocated collection of elements of the same type. 
    // Unlike arrays, vectors can change in size at runtime.
    //vec_1.push(6);  // cannot borrow as mutable
    //vec_1[0] = 10; 
    println!("Vector 1: {:?}", vec_1);

    let mut vec_2:Vec<String> = Vec::new(); // You can create an empty vector and then push elements into it.
    vec_2.push(String::from("Hello"));
    vec_2.push(String::from("World"));
    println!("Vector 1: {:?}, Vector 2: {:?}", vec_1, vec_2);

    // Tuples
    let tuple_1: (i32, f64, char) = (42, 3.14, 'R'); // A tuple is a fixed-size collection of elements of different
    // types. It can hold a heterogeneous set of values, and the types of the elements can be different.
    println!("Tuple 1: {:?}", tuple_1);

    let tuple_2: (String, bool) = (String::from("Rust"), true);
    println!("Tuple 2: {:?}", tuple_2);


    // Empty Tuple
    let empty_tuple: () = (); // The empty tuple, denoted as `()`,
    // is a special type that represents the absence of a value. It is often used as the return type of functions 
    // that do not return any meaningful value, or as a placeholder when a value is required but not needed.
    println!("Empty tuple: {:?}", empty_tuple);
 }