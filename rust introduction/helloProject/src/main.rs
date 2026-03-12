//primitive data types
//integer(int), floating point(float), boolean(bool), character(char)

//integer
//Rust has several integer types, including signed and unsigned integers of various sizes(+ and -).
// The most common integer (ONKY +)types are i32 (32-bit signed integer) and u32 (32-bit unsigned integer).
//i18, i32, i64, i128, isize(Signed integers can represent both positive and negative numbers, while unsigned integers can only represent positive numbers and zero.)
//u8, u16, u32, u64, u128, usize(Unsigned integers can only represent positive numbers and zero. The size of usize and isize depends on the architecture of the system (32-bit or 64-bit)

fn main() {
    let x: i32 = -42; // signed integer
    let y: u32 = 100; // unsigned integer
    let z: f64 = 3.14; // floating point number
    let is_rust_fun: bool = true; // boolean
    let letter: char = 'R'; // character

    println!("Integer (i32): {}", x);
    println!("Unsigned Integer (u32): {}", y);
    println!("Floating Point (f64): {}", z);
    println!("Boolean (bool): {}", is_rust_fun);
    println!("Character (char): {}", letter);

    //Compont Data types
    //Rust also has compound data types, which can group multiple values together. The most common compound data types are tuples and arrays.
    //Tuples can group together values of different types, while arrays can only contain values of the same type.
    //Tuples are defined using parentheses () and can contain values of different types.
    //Arrays are defined using square brackets [] and can only contain values of the same type.

    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // array of integers
    println!("Array: {:?}", numbers);
}

