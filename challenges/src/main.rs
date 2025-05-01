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