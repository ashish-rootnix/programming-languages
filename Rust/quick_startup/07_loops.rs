/*
    Control flow:
        Loops
        For loops
        While loops
*/

fn main()
{
    // Infinite loop
    let mut count = 0;
    loop
    {
        println!("Count: {count}");
        count += 1;
        if count >= 5
        {
            break; // Exit the loop when count reaches 5
        }
    }

    'outer: loop
    {
        println!("Inside outer loop");
        loop
        {
            println!("Inside inner loop");
            break 'outer; // Break the outer loop from the inner loop
        }
    }

    let _num: i32 = loop
    {
        break 5; // Break the loop and return a value (5 in this case)
    };


    // For loop
    let numbers = [10, 20, 30, 40, 50];
    for num in numbers.iter()
    {
        println!("Number: {num}");
    }

    let vec = vec![1, 2, 3, 4, 5];
    for (index, value) in vec.iter().enumerate()
    {
        println!("Index: {index}, Value: {value}");
    }
    
    // While loop
    let mut num = 0;
    while num < 5
    {
        println!("Num: {num}");
        num += 1;
    }

    // Common for loop syntaxes with ranges
    for i in 0..5 // This will iterate from 0 to 4 (exclusive of 5)
    {
        println!("i: {i}");
    }
    for i in 0..=5 // This will iterate from 0 to 5 (inclusive of 5)
    {
        println!("i: {i}");
    }

    for i in 5..=0 // This will not execute because the range is empty (5 to 0)
    {
        println!("i: {i}");
    } // n rust ranges are aranged in ascending order, so a range like 5..=0 is considered empty and will not execute the loop body.

    // Iterating by step size
    for i in (0..10).step_by(2) // This will iterate over
    {
        println!("i: {i}");
    }

    let _r : std::ops::RangeFull = ..; // This is a full range that can be used to iterate over an entire collection

    let _r1 : std::ops::Range<i32> = 0..3;

    // count(), contains(), find() are some of the methods that can be used with iterators in Rust to perform 
    // various operations on collections.

    // Pairs
    let pairs: Vec<(i32, &str)> = vec![(1, "one"), (2, "two"), (3, "three")];
    for (num, word) in pairs
    {
        println!("Number: {num}, Word: {word}");
    }
}