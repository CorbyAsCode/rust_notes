fn main() {
    
    /* String slices
    */
    let s = String::from("hello world");
    let hello = &s[0..5];
    // You could also use this syntax:
    // let hello = &s[..5];
    let world = &s[6..11];
    // You could also use this syntax:
    let world = &s[6..];
    println!("{} trashy {}", hello, world);

}
