use std::io;

fn main() {
    // read
    let mut input = String::new(); // Create a mutable String variable

    println!("Enter something:");

    io::stdin()
        .read_line(&mut input) // Read user input
        .expect("Failed to read input"); // Handle possible errors

    println!("You entered: {}", input); 


    // write
    let n = 3;
    println!("{n}");
    println!("{}", n);

    let s = "Hello World";
    println!("{s}");
    println!("{}", s);
}