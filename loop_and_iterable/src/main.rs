fn main()
{
    // tuple
    // can have various data types
    // 1
    let tup = (5, 3.14);
    println!("{}, {}", tup.0, tup.1);
    // 2
    let tup: (i32, f64, &str) = (500, 6.4, "Hello world");
    let (x, y, z) = tup;
    println!("x: {} \ny: {}\nz: {}", x, y, z);
    
    // array 
    // must have same data types
    // 1
    let a = ["Khorshid", "Bahar"];
    // other way
    let a: [&str; 3] = ["Afra", "Sadegh", "Anahita"];
    println!("{}", a[0]);
    // 2
    let a = [2; 3]; // [2, 2, 2]

    // loop
    // 1
    let mut i = 0;
    let a = loop {
        if i == 10 {
            break i * 2;
        }
        i += 1;
    };
    println!("{}", a); // 20

    // 2
    let mut i = 0;
    'first_loop: loop {
        let mut j = 0;
        loop {
            if j == 2 {
                break;
            }
            else if i == 2 {
                break 'first_loop;
            }
            j += 1;
        }
        i += 1;
    }

    // while
    // 1
    let array = [1, 2, 3, 4];
    let mut i = 0;
    while i < array.len() {
        println!("{}", array[i]);
        i += 1;
    }

    // for
    // 1
    let array = [1, 2, 3, 4];
    for element in array {
        println!("{}", element);
    }

    // 2
    for i in 1..5 {
        println!("{i}");
    }
}