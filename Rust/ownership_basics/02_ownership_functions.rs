/*
    Ownerships in functions:
        Functions that takes ownership of a value:
            - When a variable is passed to a function, ownership of the value is transferred to the function. 
              The function can use the value, but once the function finishes executing, 
              the value will be dropped and cannot be used outside the function.
        Returning ownership from a function:
            - If you want to return ownership of a value from a function, you can return it as part of the function's return type. 
              This allows you to transfer ownership back to the caller.
        Borrowing values in functions:
            - Alternatively, you can use references to borrow values without transferring ownership. 
              This allows you to use values in a function without taking ownership of them, and the original
                owner retains control over the value.

        take and return ownership:
            - You can also take ownership of a value in a function and then return it back to the caller. 
              This is useful when you want to perform some operations on the value and then return it for further use.
        Ownership and function parameters:
            - When you pass a value to a function, you can choose to pass it by value (transferring ownership) or by reference (borrowing). 
              The choice depends on whether you want the function to take ownership of the value or just borrow it for temporary use.
        Ownership and function return types:
            - When a function returns a value, it can either return ownership of the value or return a reference to the value. 
              Returning ownership allows the caller to take control of the value, 
              while returning a reference allows the caller to borrow the value without taking ownership.   
            
        Rust's ownership system ensures that memory is managed safely and efficiently, preventing issues like dangling
        pointers and memory leaks, while still allowing for flexible and powerful programming patterns.

*/

// Example of a function that takes ownership of a value
fn takes_ownership(s: String) {
    println!("Inside takes_ownership(): {s}");
}
fn takes_ownership_1(vec:Vec<i32>) {
    println!("Inside takes_ownership(Vec<i32>): {:?}", vec);
}

// Example of a function that takes ownership of a value and then returns it back to the caller
fn takes_ownership_and_return(s: String) -> String {
    println!("Inside takes_ownership_and_return(): {s}");
    s // Returning ownership of the string back to the caller
}

fn takes_ownership_and_return_1(vec:Vec<i32>) -> Vec<i32> {
    println!("Inside takes_ownership_and_return(Vec<i32>): {:?}", vec);
    vec // Returning ownership of the vector back to the caller
}

fn takes_ownership_and_return_2(mut vec:Vec<i32>) -> Vec<i32> {
    println!("Inside takes_ownership_and_return(Vec<i32>): {:?}", vec);
    vec.push(6); // Modifying the vector before returning it
    vec // Returning ownership of the vector back to the caller
}

// Example of a function that borrows a value without taking ownership
fn borrows_value(s: &String) {
    println!("Inside borrows_value(): {s}");
}

fn borrows_value_1(vec: &Vec<i32>) {
    println!("Inside borrows_value(Vec<i32>): {:?}", vec);
}

fn borroes_modifies_value(mut vec: &mut Vec<i32>) {
    println!("Inside borrows_modifies_value(Vec<i32>): {:?}", vec);
    vec.push(6); // Modifying the borrowed vector
    println!("Modified vector inside borrows_modifies_value(Vec<i32>): {:?}", vec);
}

/*fn borrows_value_2(mut vec: &Vec<i32>) {
    println!("Inside borrows_value(Vec<i32>): {:?}", vec);
    // vec.push(6); // This will cause a compile-time error because you cannot modify a borrowed value without taking ownership.
}*/

// fuinction that stack primitive types
fn stack_function(mut a: i32, mut b: f64, mut c: char, mut d: bool, mut e: usize)
{
    println!("Inside stack_function: a: {a}, b: {b}, c: {c}, d: {d}, e: {e}");
    a += 1;
    b *= 2.0;
    c = 'r';
    d = !d;
    e *= 2;
    println!("Modified values inside stack_function: a: {a}, b: {b}, c: {c}, d: {d}, e: {e}")
}

fn main()
{
    let s1:String = String::from("Hello Rust"); // s1 is the owner of the string "Hello Rust"
    takes_ownership(s1); // Ownership of the string is moved to the function takes_
    // println!("s1: {s1}"); // This will cause a compile-time error because s1 is no longer valid after the move.

    let vec1:Vec<i32> = vec![1, 2, 3, 4, 5]; // vec1 is the owner of the vector [1, 2, 3, 4, 5]
    takes_ownership_1(vec1); // Ownership of the vector is moved to the function takes_ownership_1
    // println!("vec1: {vec1}"); // This will cause a compile-time
    // error because vec1 is no longer valid after the move.

    let s2: String = String::from("Hello Rust"); // s2 is the owner of the string "Hello Rust"
    let s3: String = takes_ownership_and_return(s2); // Ownership of the string is moved to the function takes_ownership_and_return_1, and then returned back to s3
    println!("s3: {s3}"); // s3 is now the owner of the string "Hello Rust"

    let vec2: Vec<i32> = vec![1, 2, 3, 4, 5]; // vec2 is the owner of the vector [1, 2, 3, 4, 5]
    let vec3: Vec<i32> = takes_ownership_and_return_1(vec2); //
    println!("vec3: {vec3:?}"); // vec3 is now the owner of the vector [1, 2, 3, 4, 5]

    let vec4: Vec<i32> = vec![1, 2, 3, 4, 5]; // vec4 is the owner of the vector [1, 2, 3, 4, 5]
    let vec5: Vec<i32> = takes_ownership_and_return_2(vec4); // vec5 is now the owner of the vector [1, 2, 3, 4, 5, 6]
    println!("vec5: {vec5:?}"); // vec5 is now the owner of the vector [1, 2, 3, 4, 5, 6]

    let s4: String = String::from("Hello Rust"); // s4 is the owner of the string "Hello Rust"
    borrows_value(&s4); // s4 is borrowed by the function borrows_value
    println!("s4: {s4}"); // s4 is still valid and can be used because it was only borrowed, not moved.

    let vec4: Vec<i32> = vec![1, 2, 3, 4, 5]; // vec4 is the owner of the vector [1, 2, 3, 4, 5]
    borrows_value_1(&vec4); // vec4 is borrowed by the function borrows_value
    println!("vec4: {vec4:?}"); // vec4 is still valid and can
    // be used because it was only borrowed, not moved.   

    let mut vec6: Vec<i32> = vec![1, 2, 3, 4, 5]; // vec6 is the owner of the vector [1, 2, 3, 4, 5]
    borroes_modifies_value(&mut vec6); // vec6 is mutably borrowed
    println!("vec6 after modification: {vec6:?}"); // vec6 is still valid and can be used because it was only borrowed, not moved.
    // However, since it was mutably borrowed, we were able to modify it within the
    // borroes_modifies_value function, and the changes are reflected when we print vec6 after the function call.
    
    let a: i32 = 10;
    let b: f64 = 3.14;
    let c: char = 'R';
    let d: bool = true;
    let e: usize = 5;
    println!("Before calling stack_function: a: {a}, b: {b}, c: {c}, d: {d}, e: {e}");
    stack_function(a, b, c, d, e); // Primitive types are copied when passed to a function, so the original values of a, b, c, d, and e remain unchanged after the function call.
    println!("After calling stack_function: a: {a}, b: {b}, c: {c}, d: {d}, e: {e}");
    // Because copy of primitive types does not involve ownership transfer, Copy by value is performed, and the original 
    // variables a, b, c, d, and e remain valid and unchanged after the function call.
    // If we had passed a non-primitive type (like String or Vec) to the function, ownership would have been transferred, 
    // and the original variables would no longer be valid after the function call.
    // However, since we are passing primitive types (i32, f64, char, bool, usize), they implement the Copy trait,
    // which means that they are copied rather than moved when passed to the function.
    // Therefore, the original variables a, b, c, d, and e remain valid
    // and unchanged after the function call, because they are copied rather than moved.

    //  the original variables a, b, c, d, and e are still valid and can be used after the function call.   

}    
