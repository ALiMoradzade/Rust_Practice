fn main() {
    let number = 5;
    let allow_even = false;

    // if else
    if number % 2 == 0 && allow_even {
        println!("Correct");
    } else if number % 2 == 1 && !allow_even {
        println!("Incorrect");
    } else {
        println!("Never used!");
    }

    // match
    match (number % 2, allow_even) {
        (0, true) => println!("Correct"),
        (1, false) => println!("Incorrect"),
        _ => println!("Never used!"),
    }
}
