fn main() {
    greet();

    let a: i32 = 2;
    let b: i32 = 7;
    println!("The sum of {} and {} is {}", a, b, add(a, b));

    check_scope();

    println!("Output of a pure function: {}", generate_sha_256_hash("Hello, World!"));

    write_to_file("Dia duit, a chara!", "test.txt")
}

fn greet() {
    println!("Hello, world!");
}

// return type denoted with '-> type'
fn add(a: i32, b: i32) -> i32 {
    return a + b; // alternatively, we can remove the 'return' and the semicolon
}

// in Rust, the last expression in a block
// is considered the return value of the block

fn check_scope() { // Rust uses snake_case
    let x = {
        let y = 5;
        y + 1 // this expression will be the value of `x`
    };
    println!("The value of x is: {}", x);
}

// Pure Functions
/*
    Pure functions do not modify external state
    and always produce the same output for a given
    input. They do not cause side-effects.
*/

use sha256::digest;

fn generate_sha_256_hash(input: &str) -> String {
    let hash = digest(input.as_bytes());
    hash
}

// here is an example of a function with side-effects
use std::fs::File;
use std::io::Write;
fn write_to_file(data: &str, filename: &str) {
    let mut file = File::create(filename).expect("Failed to create file");
    file.write_all(data.as_bytes()).expect("Failed to write to file");
}

// Function overloading
/*
    Rust does not support function overloading.
*/
