fn main() {
    let my_quarter = Coin::Quarter(UsState::Alaska);
    let my_penny = Coin::Penny;
    let my_quarter_value = value_in_cents(my_quarter);
    let my_penny_value = value_in_cents(my_penny);
    println!("quarter: {}, penny: {}", my_quarter_value, my_penny_value);

    // Using Option
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);

    // Long version of a match
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // Shorter version of above that only matches one value and 
    // ignores the rest
    if let Some(3) = some_u8_value {
        println!("three");
    }

    // This version matches one value and does something
    // with the rest
    if let Some(3) = some_u8_value {
        println!("three")
    } else {
        println!("Something else");
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

