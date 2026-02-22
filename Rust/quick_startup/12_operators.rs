/*
    Operators:
        - Arithmetic Operators
        - Comparison Operators
        - Logical Operators
        - Assignment Operators
        - Bitwise Operators
        - Other Operators
*/

fn main()
{
    // Arithmetic Operators
    let a = 10;
    let b = 5;
    println!("a + b = {}", a + b); // Addition
    println!("a - b = {}", a - b); // Subtraction
    println!("a * b = {}", a * b); // Multiplication
    println!("a / b = {}", a / b); // Division
    println!("a % b = {}", a % b); // Modulus

    // Comparison Operators
    println!("a == b: {}", a == b); // Equal to
    println!("a != b: {}", a != b); // Not equal to
    println!("a > b: {}", a > b);   // Greater than
    println!("a < b: {}", a < b);   // Less than
    println!("a >= b: {}", a >= b); // Greater than or equal to
    println!("a <= b: {}", a <= b); // Less than or equal to

    // Logical Operators
    let x = true;
    let y = false;
    println!("x && y: {}", x && y); // Logical AND
    println!("x || y: {}", x || y); // Logical OR
    println!("!x: {}", !x);         // Logical NOT

    // Assignment Operators
    let mut c = 10;
    c += 5; // Equivalent to c = c + 5
    println!("c after += 5: {}", c);

    // Bitwise Operators
    let d = 0b1010; // 10 in binary
    let e = 0b1100; // 12 in binary
    println!("d & e: {:b}", d & e); // Bitwise AND
    println!("d | e: {:b}", d | e); // Bitwise OR
    println!("d ^ e: {:b}", d ^ e); // Bitwise XOR
    println!("!d: {:b}", !d);       // Bitwise NOT
    println!("d << 1: {:b}", d << 1); // Left shift
    println!("d >> 1: {:b}", d >> 1); // Right shift    

    // Other Operators
    let f:i32 = 5;
    let g:u32 = 3;
    println!("f.pow(g): {}", f.pow(g)); // Exponentiation
    println!("f as f64: {}", f as f64); // Type casting

    // Operator precedence
    let result = a + b * c; // Multiplication has higher precedence than addition
    println!("Result of a + b * c: {}", result);
    
    let result = (a + b) * c; // Parentheses change the order of evaluation
    println!("Result of (a + b) * c: {}", result);

    let result = a + b - c * d / e; // Operator precedence in a complex expression
    println!("Result of a + b - c * d / e: {}", result);

    // Associativity of operators
    let result = a - b - c; // Subtraction is left-associative, so it is evaluated as (a - b) - c
    println!("Result of a - b - c: {}", result);

    let result = a / b / c; // Division is left-associative, so it is evaluated as (a / b) / c
    println!("Result of a / b / c: {}", result);

    let result = a * b * c; // Multiplication is left-associative, so it is evaluated as (a * b) * c
    println!("Result of a * b * c: {}", result);

    // Short-circuiting behavior of logical operators
    let result = x && (y || x); // The expression y || x is evaluated first, and since y is false, the result of y || x is true. Then, x && true is evaluated, which results in true.
    println!("Result of x && (y || x): {}", result);

    let result = y || (x && y); // The expression x && y is evaluated first, and since x is true and y is false, the result of x && y is false. Then, y || false is evaluated, which results in false.
    println!("Result of y || (x && y): {}", result);

    // Explicitly specifying operator precedence using parentheses
    let result = (a + b) * (c - d); // The expressions a + b and c - d are evaluated first due to the parentheses, and then their results are multiplied
    println!("Result of (a + b) * (c - d): {}", result);

    let result = a + (b * c) - (d / e); // The expressions b * c and d / e are evaluated first due to the parentheses, and then their results are added and subtracted from a
    println!("Result of a + (b * c) - (d / e): {}", result);

    // Explicit boolean expressions to control operator precedence
    let result = (x && y) || (x || y); // The expressions x && y and x || y are evaluated first due to the parentheses, and then their results are combined using the logical OR operator
    println!("Result of (x && y) || (x || y): {}", result);

    let result = !(x && y) || (x && !y); // The expressions x && y and x && !y are evaluated first due to the parentheses, and then their results are combined using the logical OR operator
    println!("Result of !(x && y) || (x && !y): {}", result);

    // Operator overloading
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // The + operator is overloaded for String, and it concatenates the two strings. Note that s1 is moved and can no longer
    // be used after this point, while s2 is borrowed and can still be used.
    println!("Concatenated string: {}", s3);

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let v3 = v1.iter().chain(v2.iter()).cloned().collect::<Vec<i32>>(); // The + operator is not defined for vectors, but we can use the iter() method to create an iterator over the elements of the vectors, and then use the chain()
    // method to combine the iterators, followed by the cloned() method to create a new vector with the combined elements.
    println!("Combined vector: {:?}", v3);
    
}   