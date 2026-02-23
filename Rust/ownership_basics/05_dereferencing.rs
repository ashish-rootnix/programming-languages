/*
    Derefrencing is the process of accessing the value that a reference points to. 
    In Rust, you can dereference a reference using the * operator. 
    When you dereference a reference, you get access to the value it points to, allowing you to read or modify it.

    Dereferencing is an important concept in Rust because it allows you to work with references and borrowed data without taking ownership of it.
    When you dereference a reference, you can read the value it points to or modify it if you have a mutable reference.
    Rust's ownership and borrowing rules ensure that dereferencing is safe and does not lead to issues like dangling pointers or data races.
    When you dereference a reference, Rust checks that the reference is valid and that you have the appropriate permissions to access or modify the value it points to.
    
    Derefrencing of stack allocated types
    Dereferencing of heap allocated types

*/

fn main()
{
    // Derefrencing stack allocated types
    let mut x: i32 = 10; // x is a stack allocated integer
    let ref_x: &mut i32 = &mut x; // ref_x is a reference to x
    let dref_copy = *ref_x; // Dereferencing ref_x to get a copy of the value of x
    //println!("x: {x}, ref_x: {ref_x}, dref_copy: {dref_copy}"); // x: 10, ref_x: 10, dref_copy: 10
    *ref_x += 5; // Modifying the value of x through the mutable reference ref_x
    println!("After modifying through ref_x: x: {x},  dref_copy: {dref_copy}"); // After modifying through ref_x: x

    // Owned type
    let mut s: String = String::from("Hello Rust"); // s is a heap allocated string
    let ref_s: &mut String = &mut s; // ref_s is a reference
    let _dref_copy_s = (*ref_s).clone(); // Dereferencing ref_s to get a copy of the value of s
    // println!("s: {s}, ref_s: {ref_s}, dref_copy_s : {dref_copy_s}"); // s: Hello Rust, ref_s: Hello Rust, dref_copy_s: Hello Rust

    let mut heap_data: Vec<i32>  = vec![1, 2, 3, 4, 5]; // heap_data is a heap allocated vector
    let ref_heap_data: &mut Vec<i32> = &mut heap_data; //
    
    //let _dref_copy_1 = *ref_heap_data; // Error: cannot move out of dereference of `&mut Vec<i32>`
    ref_heap_data.push(6); // Modifying the vector through the mutable reference ref_heap_data
    (*ref_heap_data).push(7); // Modifying the vector through dereferencing ref_heap_data

    let dref_copy_heap_data = (*ref_heap_data).clone(); // Dereferencing ref_heap_data to get a copy of the value of heap_data
    println!("heap_data: {heap_data:?}, dref_copy_heap_data: {dref_copy_heap_data:?}"); // heap

}