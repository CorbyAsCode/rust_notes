fn main() {

    // if conditional
    let number = 3;
    if number < 5 {
        println!("number is less than 5");
    } else {
        println!("number is greater than 5");
    }

    // if else conditional
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
       println!("number is divisible by 3");
    } else if number % 2 == 0 {
       println!("number is divisible by 2");
    } else {
       println!("number is not divisible by 4, 3, or 2");
    }

    // Set variable conditionally
    let number = if number > 5 {
        number * 10
    } else {
        1000
    };

    println!("number: {}", number);
}
