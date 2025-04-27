// SCOPE
/*
In Rust, each variable has a specific scope in which it's valid
and can be accessed. The scope of a variable is determined by the 
block in which it's declared. A block is a piece of code that's 
surrounded by curly braces {}.

Whenever a variables goes out of scope, it means that the variable 
has been dropped, meaning that the memory that was allocated for 
the variable is freed and the variable can't be accessed anymore.
 */

// Constants
// let k = 10; // compiler error, can't use let outside of a function
const MAX_POINTS: u32 = 100_000; // can never be mutable
// when a variable is declared with const, it will be inlined directly
// in the program's binary. It will neither be stored in the stack,
// nor in the heap.
const PI: f32 = 3.14;
const AUTHOR: &str = "Kevin Glick";

fn main() {

    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    println!("The value of PI is: {}", PI);
    println!("The author is: {}", AUTHOR);

    // declare a local variable
    let x = 5;

    // x = 10; // compiler error

    println!("The value of x is: {}", x);

    // mutable variables
    let mut y = 5; // y is mutable

    println!("The value of y is: {}", y);

    y = 10;
    println!("The value of y is now {}", y);

    // Shadowing
    let b = 5;
    let b = b + 1;
    let b = b * 2;

    println!("The value of b is: {}", b);
}
