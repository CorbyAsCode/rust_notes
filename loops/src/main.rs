fn main() {
    /* Infinite loop
    loop {
        do something
    }
    */

    // Loop with break and return value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // return counter * 2 before exiting the loop
        }
    };

    println!("result: {}", result);

    // While loop
    let mut number = 10;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("LIFTOFF!");

    // For loop
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
  
    for element in a.iter() {
        println!("the value is {}", element);
    }

    // For loop using a range in reverse order
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!");
}

