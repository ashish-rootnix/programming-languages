/*
    Ownership BAsic:
        Each value can have only one owner at a time.
        When the owner goes out of scope, the value will be dropcleaned up.
        Ownership can be transferred (moved) or borrowed (references).
        Ownership is a compile-time feature that ensures memory safety without a garbage collector.
*/
#[allow(unused_variables)]
fn main()
{
    let s1: String = String::from("Hello Rust"); // S1 is the owner of the string "Hello Rust"
    println!("s1: {s1}");
    let s2: String = s1; // Ownership of the string is moved from S1 to S2. S1 is no longer valid.
    println!("s2: {s2}");
    // println!("s1: {s1}"); // This will cause a compile-time error because s1 is no longer valid after the move.

    let s3: String = String::from("Hello Rust"); // S3 is the owner of the string "Hello Rust"
    println!("s3: {s3}");
    let s4: &String = &s3; // S4 is a reference to S3. S3 is still the owner of the string, and S4 is borrowing it.
    println!("s4: {s4}");
    println!("s3: {s3}"); // S3 is still valid and can be used because S4 is only borrowing it, not taking ownership.
}

/*
    Stack: LIFO (Last In, First Out) data structure that is used for storing local variables and function call frames. 
        It is fast and efficient, but has a limited size. 
        Values stored on the stack are automatically deallocated when they go out of scope.
        Data that has fixed size at compile time is stored on the stack, such as integers, floats, and fixed-size arrays.

    Heap: A region of memory that is used for dynamic memory allocation. 
        It is slower than the stack but can grow and shrink at runtime.
        Values stored on the heap must be manually allocated and deallocated using smart pointers or 
        other memory management techniques.
        Unknown size data at compile time, such as String and Vec, is stored on the heap.

    Static: Binary program and data that is embedded in the executable file. It is allocated at compile time and has a fixed size. 
        Static variables are stored in the static memory region and have a 
        'static lifetime, meaning they live for the entire duration of the program.

    Ownership rules:
        Each value in Rust has a variable that is called its owner.
        There can only be one owner at a time.
        When the owner goes out of scope, the value will be dropped (deallocated).
        Ownership can be transferred (moved) or borrowed (references).
        Ownership is a compile-time feature that ensures memory safety without a garbage collector.

        As like C++ deep copy and shallow copy, Rust has move semantics and borrowing semantics.
        Move semantics: When a value is moved, the ownership of the value is transferred from one variable to another. 
        The original variable is no longer valid after the move, and any attempt to use it will result in a compile-time error.
        Borrowing semantics: When a value is borrowed, a reference to the value is created. 
        The original variable remains valid, and the reference can be used to access the value.
        Rust enforces these ownership rules at compile time, which helps prevent common programming errors such as dangling pointers, 
        double frees, and memory leaks.

    Zero runtime overhead: Rust's ownership system is designed to have zero runtime overhead.
        It is a compile-time feature that does not require any additional runtime checks or garbage collection,
        which allows Rust to achieve high performance while still ensuring memory safety.
    
    Clone: The Clone trait allows for explicit copying of data. When you call the clone() method on a value, 
        it creates a new instance of that value with the same data. This is different from move semantics, 
        where ownership is transferred without creating a new instance. 
        loning can be more expensive than moving because it involves creating a new instance and copying the data,
        but it can be useful when you need to retain ownership of the original value while creating a new one.

    Scopes and Lifetimes:
        Scope: A scope is a region of code where a variable is valid. 
        When a variable goes out of scope, it is dropped and its memory is deallocated.
        Lifetimes: A lifetime is a construct that the Rust compiler uses to track how long references are valid. 
        It ensures that references do not outlive the data they point to, preventing dangling references and ensuring memory safety.
        The 'static lifetime is a special lifetime that indicates that a reference is valid for the entire duration of the program. 
        It is often used for string literals and other data that is embedded in the binary and does not need to be deallocated. 
    
    Primitive Types: Primitive types in Rust include integers, floating-point numbers, booleans, and characters. 
        These types are stored on the stack and have a fixed size at compile time. 
        They implement the Copy trait, which means that when you assign a primitive value to another variable, 
        it creates a copy of the value rather than moving ownership. 
        For example, when you assign an integer to another variable, it creates a copy of the integer value, 
        and both variables can be used independently without affecting each other. 
        This is different from non-primitive types, such as String and Vec, which are stored on the heap and implement the Move trait, 
        meaning that when you assign them to another variable, ownership is transferred rather than copied.
    
    Compound Types: Compound types in Rust include arrays, vectors, tuples, and the empty tuple. 
        These types can contain multiple values and can be stored on the stack or the heap depending on their size and usage. 
        Arrays and tuples are typically stored on the stack, while vectors are stored on the heap. 
        
        The ownership and borrowing rules apply to compound types as well, and they can implement 
        the Clone trait if their elements also implement Clone. 
        
        For example, a vector of integers can be cloned if the integers implement Clone, but a vector of non-primitive types 
        that do not implement Clone cannot be cloned without implementing Clone for the contained types.   

    Data Ownership and Memory Management:
        Rust's ownership system is designed to ensure memory safety without a garbage collector. 
        
        When a value goes out of scope, it is automatically dropped and its memory is deallocated. 
        
        This means that Rust does not have a garbage collector like other languages, and it relies on the ownership system to manage memory. 
        
        The ownership rules ensure that there are no dangling pointers, double frees, or memory leaks, which can lead to safer 
        and more efficient code. 
        
        Rust also provides smart pointers, such as Box, Rc, and Arc, which allow for more flexible ownership and borrowing patterns 
        while still ensuring memory safety. 
        
        These smart pointers can be used to manage heap-allocated data and enable features like reference counting and shared ownership. 
        
        Overall, Rust's ownership system is a powerful tool for managing memory and ensuring safety, and it is one of the key features 
        that sets Rust apart from other programming languages.

    User defined types: In Rust, you can define your own types using structs and enums. 
        Structs are used to create custom data structures that can hold multiple values of different types, 
        while enums are used to define a type that can be one of several variants. 
        
        Both structs and enums can implement the Clone trait if their fields also implement Clone, 
        allowing for explicit copying of data. 
        
        The ownership and borrowing rules apply to user-defined types as well, and they can be stored on the stack or 
        the heap depending on their size and usage. 
        
        User-defined types can also have methods associated with them, allowing you to define behavior for your custom types.
*/