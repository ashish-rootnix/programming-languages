/*
    Borrowing in functions:
        - Alternatively, you can use references to borrow values without transferring ownership. 
          This allows you to use values in a function without taking ownership of them, and the original
            owner retains control over the value.
        - When you borrow a value, you create a reference to it.
            The reference can be either immutable (read-only) or mutable (read-write), depending on how you declare it.
        - Rust's borrowing rules ensure that references are always valid and that there are no data races or other unsafe behaviors when accessing shared data.
        - When you borrow a value, you can use it within the scope of the borrow, but you cannot modify it unless you have a mutable reference. 
          This helps prevent bugs and ensures that data is accessed safely and predictably.
        - 
        Function that immutably borrows a value:
            - You can define a function that takes an immutable reference to a value, allowing you to read the value without modifying it. 
              This is useful when you want to use a value in a function without taking ownership of it.
        Function that mutably borrows a value:
            - You can also define a function that takes a mutable reference to a value, allowing you to modify the value within the function. 
              This is useful when you want to perform some operations on a value without taking ownership of it, but still need to modify it.
        Function that not uses borrowing but returns ownership:
            - You can define a function that takes ownership of a value, performs some operations on it, and then returns ownership of the value back to the caller. 
              This is useful when you want to perform some operations on a value and then return it for further use, while still allowing the caller to retain control over the value.
        Function uses mix type of borrowing and ownership:
            - You can also define a function that takes ownership of some values and borrows others, allowing you to perform operations on both owned and borrowed data within the same function. 
              This is useful when you want to work with a combination of owned and borrowed data in a single function, while still adhering to Rust's ownership and borrowing rules.
*/

fn borrows_value(vec: &Vec<i32>) {
     println!("Inside borrows_value(Vec<i32>): {:?}", vec);

}

fn borroes_modifies_value(vec: &mut Vec<i32>) {
    println!("Inside borrows_modifies_value(Vec<i32>): {:?}", vec);
    vec.push(6); // Modifying the borrowed vector
    println!("Modified vector inside borrows_modifies_value(Vec<i32>): {:?}", vec);
}

fn not_use_borrows_and_returns_ownership(vec: Vec<i32>) -> Vec<i32> {
    println!("Inside not_use_borrows_and_returns_ownership(Vec<i32>): {:?}", vec);
    let mut modified_vec = vec; // Taking ownership of the vector
    modified_vec.push(6); // Modifying the vector
    modified_vec // Returning ownership of the modified vector back to the caller
}

fn mix_borrow_and_ownership(vec: Vec<i32>, s: &mut String) -> Vec<i32> {
    println!("Inside mix_borrow_and_ownership(): vec: {:?}, s: {s}", vec);
    let mut modified_vec = vec; // Taking ownership of the vector
    modified_vec.push(6); // Modifying the vector
    println!("Modified vector inside mix_borrow_and_ownership(): {:?}", modified_vec);

    s.push_str(" - modified"); // Modifying the borrowed string
    println!("Modified string inside mix_borrow_and_ownership(): {s}");
    modified_vec // Returning ownership of the modified vector back to the caller
}

/*fn give_ownership() -> &Vec<i32> // Error: expected named lifetime parameter, not a valid reference
{ 
    let vec = vec![1, 2, 3, 4, 5]; // vec is the owner of the vector [1, 2, 3, 4, 5]
    &vec // Returning a reference to the vector (borrowing it)
}*/

fn main()
{
    let vec4: Vec<i32> = vec![1, 2, 3, 4, 5]; // vec4 is the owner of the vector [1, 2, 3, 4, 5]
    borrows_value(&vec4); // vec4 is borrowed by the function borrows_value
    println!("vec4: {vec4:?}"); // vec4 is still valid and can

    // be used because it was only borrowed, not moved.
    let mut vec5: Vec<i32> = vec![1, 2, 3, 4, 5]; // vec5 is the owner of the vector [1, 2, 3, 4, 5]
    borroes_modifies_value(&mut vec5); // vec5 is mutably borrowed
    println!("vec5 after modification: {vec5:?}"); // vec5 is still valid and can be used because it was only borrowed, not moved.
    // However, since it was mutably borrowed, we were able to modify it within the borroes_modifies_value function, and the changes are reflected when we print vec5 after the     

    let vec6: Vec<i32> = vec![1, 2, 3, 4, 5]; // vec6 is the owner of the vector [1, 2, 3, 4, 5]
    let modified_vec6 = not_use_borrows_and_returns_ownership(vec6); //
    //vec6 is moved into the function not_use_borrows_and_returns_ownership, and ownership is returned back to modified_vec6
    println!("vec6 after modification: {modified_vec6:?}"); // modified_vec6 is the new owner of the modified vector, and we can use it to access the modified data.

    let mut s: String = String::from("Hello Rust"); // s is the owner of the string "Hello Rust"
    let modified_vec7 = mix_borrow_and_ownership(vec![1, 2, 3, 4, 5], &mut s); // The vector is moved into the function mix_borrow_and_ownership, and ownership is returned back to modified_vec7. The string s
    // is borrowed by the function mix_borrow_and_ownership, allowing us to modify it within the function without taking ownership of it.
    println!("modified_vec7: {modified_vec7:?}, modified string s: {s}"); // modified_vec7 is the new owner of the modified vector, and we can use it
    // to access the modified data. The string s is also modified within the function, and we can see the changes when we print it after the function call.

    //let vec8 = give_ownership();
    //println!("vec8: {vec8:?}");
}