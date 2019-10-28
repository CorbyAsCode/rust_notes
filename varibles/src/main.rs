fn main() {
    
    // Constant
    const MAX_POINTS: u32 = 100_000;
    println!("max points: {}", MAX_POINTS);

    // Mutability
    let mut x = 5;
    println!("x: {}", x);
    x = 6;
    println!("x: {}", x);

    // Shadowing
    let y = 5;
    println!("y before shadow: {}", y);
    let y = y + 5;
    let y = y * 2;
    println!("y after shadow: {}", y);

    // Shadowing with type change
    let spaces = "    ";        // This is a string
    println!("spaces: x{}x", spaces);
    let spaces = spaces.len();  // This is an integer
    println!("spaces: x{}x", spaces);

    // Tuples
    let tup: (u32, f64, u8) = (56, 2.345, 5);
    let the_32bit = tup.0;
    let the_float = tup.1;
    let the_8bit = tup.2;
    println!("the_32bit: {}, the_float: {}, the_8bit: {}", the_32bit, the_float, the_8bit);
    let (tup_x, tup_y, tup_z) = tup;
    println!("tup_x: {}, tup_y: {}, tup_z: {}", tup_x, tup_y, tup_z);

    // Arrays
    let arr = [1, 2, 3, 4, 5];
    println!("first element of immutable arr: {}", arr[0]);

    let mut mut_arr = [1, 2, 3, 4, 5];
    println!("first element of mut arr: {}", mut_arr[0]);
    mut_arr[0] = 10;
    println!("first element of mut arr: {}", mut_arr[0]);

    let type_a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("first element of type_a: {}", type_a[0]);
}
