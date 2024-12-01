// Primitive data types
// int, float, bool, char

fn main() {
    // Integer
    // Rust has signed (- and -) and unsigned integer (only+) types of different sizes.
    // i8, i16, i32, i64, i128: Signed integers.
    // u8, u16, u32, u64, u128: Unsigned integers.
    // The number stand for the numbers of bits.

    let x: i32 = -42;
    let y: u32 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);
    
    // Compile error below error[E0600]: cannot apply unary operator `-` to type `u32`
    // let y: u32 = -100;
    // println!("Signed Integer: {}", y);

    // diff bet i32 (32bits) and i64 (64bits)
    // range:
    // i32 -2147483648 to 2147483647
    // i64 -9223372036854775808 to 9223372036854775807
    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;
    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);

    // error: literal out of range for `i32`
    // let e: i32 = 2147483648;
    // error: literal out of range for `i64`
    // let i: i64 = 9223372036854775809;
    // println!("Maximum value of i32: {}", e);
    // println!("Maximum value of i64: {}", i);

    // Floats [Floating Point Types]
    // f32, f64
    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);

    // boolean Values: true, false
    let is_snowing: bool = true;
    println!("It is snowing ? {}", is_snowing);

    // Character Type - char
    let letter: char = 'a';
    println!("first letter of the alphabet: {}", letter);
}

