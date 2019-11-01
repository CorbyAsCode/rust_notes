fn main() {

    /* Ownership rules
        1. Each value has a variable that's called its owner.
        2. There can only be one owner at a time.
        3. When the owner goes out of scope, the value is dropped.
    */

    /* String literal vs String 
       This is a string literal and is immutable and stored 
       on the stack:
       let s = "test";
    */
    
    // This is a String type and is mutable and is stored on the heap
    let mut s = String::from("test");  
                                       
    s.push_str("ing");
    println!("{}", s);

    /* This produces a new pointer to the same memory in heap.
       At this point `s` has been moved to `s2`, ownership 
       has been transferred.
    */
    let s2 = s;
    println!("s2: {}", s2);

    /* There is a way to make a new copy of data in heap using
       the clone method.  This is a slow process and consumes
       more memory.
    */
    let str1 = String::from("my string");
    let str2 = str1.clone();
    println!("str1: {}, str2: {}", str1, str2);

    /* Scalars are always stored on the stack because they
       have a known size at compile time.
       When you copy one scalar to another both variables are in
       scope and are accessible.
    */
    let a = 5;
    let b = a;
    println!("a: {}, b: {}", a, b);

    /* Ownership regarding functions
       The same rules as Strings and scalars apply here
    */
    
    let fn_str = String::from("this is my function string");
    takes_ownership(fn_str);  // fn_str is now owned by the function
                              // and will go out of scope when the 
                              // function finishes
    let x = 5;
    makes_copy(x);  // x goes out of scope at the end of main(), 
                    // because it is copied within the stack
    println!("x: {}, after it's used in makes_copy()", x);

    /* What if we don't want a function to take full ownership of a variable?
       You need to pass a reference to the variable to the function
    */
    let s_ref = String::from("hi");
    let len = calculate_length(&s_ref);
    println!("The length of '{}' is {}.", s_ref, len);

    /* Normally you can't change the value of a variable via the
       reference, but you can in Rust by changing the reference
       to be mutable.  You can only have 1 mutable reference to a
       variable in a scope.  This prevents data race conditions when a 
       synchronizer like a semaphore isn't being used.
       You also can't reference a variable as mutable and immutable in
       the same scope and use both at the same time because the data 
       can be changed and the immutable reference isn't expecting 
       anything to be changed.
    */
    let mut s_ref = String::from("hi");
    println!("s_ref: {}, before change()", s_ref);
    change(&mut s_ref);
    println!("s_ref: {}, after change()", s_ref);

    /* You can have immutable references before mutable ones as long
       as the immutable references aren't being used at the same time.
    */
    let mut s = String::from("hi");
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);
    let r3 = &mut s;
    println!("r3: {}", r3);

}

fn takes_ownership(s: String) {
    println!("s: {}, and is owned by takes_ownership()", s);
}

fn makes_copy(x: i32) {
    println!("x: {}, and is still accessible within main()", x);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" world");
}
