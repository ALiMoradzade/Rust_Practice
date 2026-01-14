fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let mut s = String::new();
    
    // s = s1; // value is moved
    // s = s1.as_str().parse().unwrap(); // gets str and converts it to String
    
    // s = s1 + " " + s2.as_str(); // s1 is moved and did String + str
    s = s + &s1 + &s2; // s1 is moved and did String + str
    
    println!("{}", s);
}
