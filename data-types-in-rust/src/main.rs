fn main() {
    let x = 5; // Type is inferred as i32
    let y: i64 = 10; // Rust will use the type i64 for y
    // let z = "999".parse().unwrap(); // Will cause compile error
    let z: i32 = "999".parse().unwrap(); // this works

    println!("The value of z is: {}", z);

    // Subsets of types in Rust
    /*
        Rust has two main categories of types:
        1) Scalar Types - represents single value
        2) Compound Types - represents collection of values
     */

    // Scalar types
    /*
        - Integers
        - Floating-point numbers
        - Booleans
        - Characters

        Integer Types
        - 8-bit  : i8     | u8
        - 16-bit : i16    | u16
        - 32-bit : i32    | u32
        - 64-bit : i64    | u64
        - 128-bit: i128   | u128
        - arch   : isize  | usize

        Integer Literals
        - Decimal: 99_999
        - Hexadecimal: 0x1a3f
        - Octal: 0o137
        - Binary: 0b1010_1010
        - Byte (u8 only): b'Z'
     */

    let sum = 5 + 10; // Type is inferred as i32
    let difference = 95.5 - 4.3; // Rust will use the type f64 for difference
    let product = 4 * 30; // Type is inferred as i32
    let quotient = 56.7 / 32.2; // Rust will use the type f64 for the quotient
    let remainder = 43 % 5; // Type is inferred as i32

    println!("The sum is: {}", sum);
    println!("The difference is: {}", difference);
    println!("The product is: {}", product);
    println!("The quotient is: {}", quotient);
    println!("The remainder is: {}", remainder);

    // let bad = 4 * 30.4; // compiler error, different types
    let good = 4 * 30.4 as i32; // this works, but note 30.4 will be truncated to 30
    let caution = 11/10; // the result will be 1, not 1.1
    println!("The quotient is: {}", caution);

    // Floating-point numbers
    let f = 2.0; // Type is inferred as f64
    let g: f32 = 3.0; // Rust will use the type f32

    println!("The value of f is: {}", f);
    println!("The value of g is: {}", g);

    // Booleans - take only 1 byte of memory
    let is_true = true;
    let is_false: bool = false;

    println!("The value of is_true is: {}", is_true);
    println!("The value of is_false is: {}", is_false);

    // Character - specified using single quotes, 4 bytes of memory
    let c = 'z'; // inferred as char
    let z: char = '😀';
    let chinese: char = '中';
    let japanese: char = '日';

    println!("The value of c is: {}", c);
    println!("The value of z is: {}", z);
    println!("The value of chinese is: {}", chinese);
    println!("The value of japanese is: {}", japanese);

    // Compound types
    // - Arrays
    // - Tuples

    let ar = [1, 2, 3, 4, 5];
    println!("The value of ar is: {:?}", ar);

    println!("The first element of ar is: {}", ar[0]);
    println!("The second element of ar is: {}", ar[1]);
    println!("The third element of ar is: {}", ar[2]);
    println!("The fourth element of ar is: {}", ar[3]);
    println!("The fifth element of ar is: {}", ar[4]);
    // println!("The sixth element of ar is: {}", ar[5]); -- no compile error, but will cause runtime error

    // the array is stored in the stack
    // arrays should not be confused with vectors
    // vectors are flexible lists that can grow or shrink in size, while arrays are fixed

    let seasons = ["Spring", "Summer", "Fall", "Winter"]; // inferred as type [&str; 4]
    println!("The value of seasons is: {:?}", seasons);

    // Tuples
    let tup: (i32, f64, u8) = (999, 3.14, 1);
    println!("The value of tup is: {:?}", tup);
    println!("The value of x is: {}", tup.0);
    println!("The value of y is: {}", tup.1);
    println!("The value of z is: {}", tup.2);

    let (h, i, j) = tup; // using destructuring
    println!("The value of x is: {}", h);
    println!("The value of y is: {}", i);
    println!("The value of z is: {}", j);

    // Unit type
    ()
}
