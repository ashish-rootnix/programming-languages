/*
    Mutability in function parameters
*/
fn mutate_num(mut_var: &mut i32)
{
    *mut_var = 20; // Mutate the value of the variable passed to it
}

fn immutable_num(imm_var: &i32)
{
    // imm_var = 30; // Error: cannot assign to `imm_var`, which is behind a `&` reference
    println!("Inside immutable_num(): {imm_var}");
}

fn immutable_num_without_refrence(imm_var: i32)
{
    let mut _var = imm_var; // We can create a mutable variable inside the function 
    // and assign the value of the immutable parameter to it
    _var = 30; // Now we can mutate the variable inside the function
    println!("Inside immutable_num_without_refrence(): {_var}");
}

fn main()
{
    let mut num = 10; // Declare a mutable variable
    println!("Before mutation: {num}");

    // Pass the mutable variable to a function
    mutate_num(&mut num); // We need to pass a mutable reference to the variable
    println!("After mutation: {num}");

    let imm_num = 15; // Declare an immutable variable
    immutable_num(&imm_num); // Pass an immutable reference to the function
    
    //mutable_num(&mut imm_num); // Error: cannot borrow `imm_num` as mutable, as it is not declared as mutable
    immutable_num_without_refrence(imm_num); // We can pass the immutable variable by value to the function
}