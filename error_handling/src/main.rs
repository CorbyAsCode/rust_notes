use std::fs::File;
use std::io::{ErrorKind, Read};
use std::io;
use std::fs;

fn main() {
    let fh = File::open("hello.txt");

    let fh = match fh {
        Ok(f) => f,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error)
        },
    };

    // A more concise way to write the above code using closure
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // This returns the file handle or call panic!
    let f = File::open("hello.txt").unwrap();

    // This does the same as above, but prints whatever message you want
    // if it throws panic!
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // Return result from a function
    let user = read_username_from_file();
    println!("read_username_from_file: {:?}", user);

    let user2 = read_username_from_file2();
    println!("read_username_from_file2: {:?}", user2);

    let user3 = read_username_from_file3();
    println!("read_username_from_file3: {:?}", user3);
}


// This is a short way to return a Result<T, E> from a function using `?`
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// This is an even shorter way to return a Result<T, E>
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// And this is the absolute shortest way to read a file
// and return the string or an error
fn read_username_from_file3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
