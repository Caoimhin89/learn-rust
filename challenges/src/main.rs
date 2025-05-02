fn main() {
    // numerical conversion
    let num_i32: i32 = 5;
    println!(
        "Converted {} i32 to {} u32",
        num_i32,
        numerical_type_conversion(num_i32),
    );

    // mathematical operations
    math_operations(5, 9);

    // sum of array
    let arr_to_sum: &[i32] = &[1, 2, 3, 4, 5];
    let summed_arr = sum_array(arr_to_sum);
    println!("Summed array is {}", summed_arr);
}

// converting numerical types
/*
    Since we have so many different numerical types in Rust (e.g., i32, u32, f32, etc.), it is common to need to convert between them.

    In Rust, converting between numerical types is often done using the as keyword. This challenge focuses on using as to convert an i32 to a u32. While this conversion is straightforward, it is crucial to understand the implications and usage of the as keyword for safe and efficient type casting in Rust.

    ads via Carbon
    One platform to empower Dev, Sec, and Ops teams. Start building secure software today.
    ads via Carbon
    The as keyword
    The as keyword in Rust is used for casting between different types. It is commonly used to convert between numerical types. The as keyword is used to convert a value from one type to another, as long as the conversion is valid and does not result in data loss or overflow.

    Your task
    Implement a function called numerical_type_conversion that takes an i32 as input and returns it as a u32. You should use the as keyword to perform this conversion.

    Requirements
    The function should take an i32 as input and return a u32.
    You must use the as keyword to perform the type conversion.
*/
pub fn numerical_type_conversion(n: i32) -> u32 {
    n as u32
}

// Mathematical operations
/*
    This challenge is about basic mathematical operations. You will be given 2 numbers a and b. You need to perform the following operations:

    1.) Sum of a and b
    2.) Difference of a and b
    3.) Multiplication of a and b
    4.) Division of a and b

    You need to return a tuple containing the results of the above operations in the same order. (sum, difference, multiply, divide)
*/
pub fn math_operations(a: i32, b: i32) -> (i32, i32, i32, i32) {
    let sum: i32 = a + b;
    let difference: i32 = a - b;
    let product: i32 = a * b;
    let quotient: i32 = a / b;

    (sum, difference, product, quotient)
}

// Sum of Array
/*
    Arrays are a fundamental data structure in Rust that allow you to store a fixed-size collection of elements of the same type. A common operation is to calculate the sum of all elements in an array.

    In this challenge, you will implement a function to calculate the sum of elements in an array of integers i32.

    Design and Development tips in your inbox. Every weekday.

    Your task:
    You need to implement the function sum_array(arr: &[i32]) -> i32 that takes a slice of integers and returns the sum of all elements.

    Requirements:
    The sum_array function should return the sum of all elements in the array.
*/
pub fn sum_array(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

// Tuples
/*
    Tuples are a simple and versatile data structure in 
    Rust that allow you to group multiple values of 
    different types into a single compound value. 
    They are particularly useful for returning multiple 
    values from a function.

    Tuples can return multiple values of different types, 
    which is not possible with arrays or slices. 
    For example a tuple could be (i32, f64, String) 
    which contains an integer, a float, and a string.

    In this challenge, you will implement a function that 
    takes three arguments of different types and returns 
    them as a tuple.

    Your task:
    You need to implement the function 
    create_tuple(a: i32, b: f64, c: &str) -> (i32, f64, String) 
    that takes an integer i32, a float f64, and 
    a string slice &str as input and returns them as a 
    tuple. The string slice should be converted into a 
    String type.

    The create_tuple function should return a tuple 
    containing the three input values in order.
    The string slice input should be converted into a 
    String before returning.
*/
pub fn create_tuple(a: i32, b: f64, c: &str) -> (i32, f64, String) {
    (a, b, c.to_string())
}

// The Unit Type
/*
    In Rust, the unit type () is a type that has exactly 
    one value, also written as (). It is used to indicate 
    the absence of a meaningful value and is often seen 
    in functions that do not return a value.

    In this challenge, you will implement a function that 
    prints a message and returns the unit type.

    Your task:
    You need to implement the function 
    print_message() -> () that prints "Hello, Rust!" 
    to the console and returns the unit type.

    Requirements:
    The print_message function should print "Hello, Rust!" 
    to the console.
    The function should return the unit type ().
*/
pub fn print_message() -> () {
    println!("Hello, Rust!");
    ()
}

// Functions
/*
    Functions are a fundamental building block in Rust, 
    as in any programming language. They allow you to 
    encapsulate logic and reuse code, making your 
    programs more modular and easier to understand. 
    In this challenge, you will define and implement a 
    series of simple functions that perform basic 
    operations.

    Your task:
    Your task is to define three functions:

    add(a: i32, b: i32) -> i32
        This function should return the sum of a and b.
    subtract(a: i32, b: i32) -> i32
        This function should return the difference between 
        a and b.
    multiply(a: i32, b: i32) -> i32
    This function should return the product of a and b.
    You need to complete these functions so that they 
    correctly perform the specified operations.
*/
pub fn add(a: i32, b: i32) -> i32 {
    // Step 1: implement addition
    a + b
}

// Step 2:
// Define a public function named `subtract`
// that accepts two arguments of type `i32`
// and returns an `i32`.
// make sure you make the function public by using the `pub` keyword.
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

// Step 3:
// Define a public function named `multiply`
// that accepts two arguments of type `i32`
// and returns an `i32`.
// make sure you make the function public by using the `pub` keyword.
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// Control Flow
/*
    Control flow is a fundamental concept in programming 
    that allows you to dictate how your code executes 
    based on certain conditions. In Rust, one of the ways 
    to control the flow of your program is by using 
    if-else statements.

    In this challenge, you will implement a function that 
    checks whether a number is positive, negative, or zero. 
    Depending on the value, your function should return a 
    corresponding string message.

    Your task:
    Your task is to complete the function 
    check_number_sign that takes an integer i32 as input 
    and returns a String indicating whether the number 
    is "positive", "negative", or "zero".

    Requirements
    - If the number is greater than zero, return "positive".
    - If the number is less than zero, return "negative".
    - If the number is equal to zero, return "zero".
*/
pub fn check_number_sign(number: i32) -> String {
    // Return `"positive"` if the number is positive.
    // Return `"negative"` if the number is negative.
    // Return `"zero"` if the number is zero.

    // Step 1:
    // Check if the number is positive.
    if number > 0 {
        String::from("positive")
    }

    // Step 2:
    // Check if the number is negative.
    else if number < 0 {
        String::from("negative")
    }

    // Step 3:
    // Handle the case where it's neither positive nor negative.
    else {
        String::from("zero")
    }
}

// Ownership
/*
    Rust's ownership model is one of its most unique and 
    powerful features, ensuring memory safety without 
    needing a garbage collector. Ownership in Rust is 
    governed by a set of rules that the compiler checks 
    at compile time. Understanding these rules is crucial 
    for writing efficient and safe Rust code.

    Ownership Basics
    In Rust, each value has a variable that's called its 
    owner. There can only be one owner at a time, and 
    when the owner goes out of scope, the value is 
    dropped. Here are the basic rules of ownership:

    - Each value in Rust has a variable that's called its 
    owner.
    - There can only be one owner at a time.
    - When the owner goes out of scope, the value will be 
    dropped (no longer valid).

    Borrowing
    Rust allows you to create references to a value, which 
    lets you access it without taking ownership. This is 
    called borrowing. Borrowing can be immutable or mutable.

    Immutable References
    You can create multiple immutable references to a 
    value, but you cannot have a mutable reference while 
    immutable references exist. This allows you to read 
    from the value without changing it.

    Using & to Create References:
    In the example above, &s1 creates an immutable reference 
    to s1. This means that calculate_length borrows s1 
    but does not take ownership of it. The & symbol is 
    used to denote a reference in Rust. This allows the 
    function to access the value without taking ownership, 
    which means s1 can still be used after the function 
    call.

    Similarly, in the function signature 
        fn calculate_length(s: &String) -> usize, &String 
    indicates that the parameter s is an immutable reference 
    to a String. This allows the function to read from 
    the String without modifying it or taking ownership.

    Challenge:
    In this challenge, you will create a function 
    calculate_length that takes an immutable reference 
    to a String, calculates its length, and returns the 
    length.

    The task is designed to help you understand the 
    concepts of ownership and immutable borrowing in Rust.

    Requirements:
    The calculate_length function should take an immutable 
    reference to the input String and return its length.
*/
pub fn calculate_length(s: &String) -> usize {
    s.len()
}