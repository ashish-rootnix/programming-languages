/*
    Basic Data Types:
        Intergers - i8, i16, i32, i64, i128, isize
        Unsigned Intergers - u8, u16, u32, u64, u128, usize
        Floating Point - f32, f64
        Boolean - bool
        Character - char
*/

fn main()
{
    // Unsigne dint:
    let unsigned_num:u8 = 255; // Unsigned integers can only represent non-negative values, and their range is from 0 to 255 for u8.
    println!("Unsigned number: {unsigned_num}");

    // Signed int:
    let signed_num:i8 = -128; // Signed integers can represent both negative and positive values, and their range is from -128 to 127 for i8.
    println!("Signed number: {signed_num}");

    // Usize and isize:
    let usize_num:usize = 100; // The usize type is an unsigned integer type that is used for indexing and measuring the size of 
    // collections. The isize type is a signed integer type that is used for pointer offsets and can represent 
    // both positive and negative values. The size of usize and isize depends on the target architecture (32-bit or 64-bit).
    let isize_num:isize = -50;
    println!("usize number: {usize_num}, isize number: {isize_num}");

    // Floating Point:
    let float_num:f64 = 3.14; // Rust has two floating-point types: f32 and f64. The f32 type is a 32-bit floating-point number,
    // while the f64 type is a 64-bit floating-point number. The default type for floating-point literals is f64.
    println!("Floating point number: {float_num}");

    // Boolean:
    let is_rust_fun:bool = true; // The bool type can only have two possible values: true and false. It is commonly used for conditional statements and logical operations.
    println!("Is Rust fun? {is_rust_fun}");

    // Character:
    let letter:char = 'R'; // The char type represents a single Unicode scalar value
    println!("Character: {letter}");

    // Type Alising
    type Age = u8; // Type aliasing allows you to create a new name for an existing type. 
    // This can be useful for improving code readability and making it easier to understand the purpose of a type.
    let my_age: Age = 25;
    println!("My age is: {my_age}");

    // Type conversion
    let a:i32 = 10;
    let _b:f64 = a as f64; // Rust provides the `as` keyword for explicit type conversion. 
    // You can use it to convert between different types, such as from an integer
}