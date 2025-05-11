use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("I need 3 numbers...\n");
    println!("Enter a (Quadratic Coefficient): ");
    io::stdin()
        .read_line(&mut a)
        .expect("Enter Number!...");
    let a: f32 = a.trim().parse().expect("Wrong Input!");


    println!("Enter b (Linear Coefficient): ");
    io::stdin()
        .read_line(&mut b)
        .expect("Enter Number!...");
    let b: f32 = b.trim().parse().expect("Wrong Input!");


    println!("Enter c (Constant Term): ");
    io::stdin()
        .read_line(&mut c)
        .expect("Enter Number!...");
    let c: f32 = c.trim().parse().expect("Wrong Input!");


    println!();

    let delta = (b.powf(2.0) - 4.0 * a * c).sqrt();
    let x1 = (-b + delta) / (2.0 * a);
    let x2 = (-b - delta) / (2.0 * a);

    println!("∆ = {}", delta);
    println!("x1 = {}", x1);
    println!("x2 = {}", x2);
}
