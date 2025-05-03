use std::io;

fn main() {
    let mut a = String::new();
   
    println!("I need 3 numbers...\n");
    println!("Enter a (Quadratic Coefficient): ");
    
    io::stdin().read_line(&mut a);

    print!("{a}")
}
