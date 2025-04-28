fn main() {
    // declare mutable var
    let mut x = 5;
    println!("var is {x}");
    // if var was immutable, you can not do this (throws error)
    x = 6;
    println!("now, var is {x}");

    // shadowing
    // (same name var but various values)
    let x: f32 = 3.1415;
    println!("var is {x}");
    {
        let x = "Hello World";
        println!("{x}");
    }
    println!("YES...I am alive {x}");

    // expression
    // (pay attention to semicolon)
    // 1
    let y = {
        let x = 3;
        x + 1
    };
    // 2
    let number: &str = if y % 2 == 0 { "Even" } else { "Odd" };
    println!("{0}",number); // Even
}