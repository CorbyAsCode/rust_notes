fn main() {
    println!("Hello, world!");

    // Function parameters
    another_function(5, "dumbass".to_string());

    // Set variable equal to expression
    let y = {
        let x = 3;
        x + 1
    };
    println!("y: {}", y);

    // Function with a return value
    let x = return_something();
    println!("x: {}", x);
    println!("return_something(): {}", return_something());
}

// Function parameters
fn another_function(x: u32, y: String) {
    println!("{} = {}.", y, x);
}

fn return_something() -> i32 {
    5
}
