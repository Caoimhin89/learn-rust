// The Stack
/*
    a linear data structure that stores data in order,
    the first data being stored in the stack is the last
    piece of data that is removed later. LIFO (Last In First Out).

    In the Stack, the size and location of every datum is known
    at compile time. Accessing data from the stack is therefore
    much faster because allocation and de-allocation is not required.
    Since the data is fixed, data can not shirnk or grow at runtime.
*/

// Order of Events for Example Program
/*
    Stack:
    - add(3, 4)
    - do_math(3, 4)
    - main()

    then add(3, 4) finishes executing and is removed
    and subtract(3, 4) is added to the stack

    Stack:
    - subtract(3, 4)
    - do_math(3, 4)
    - main()

    then subtract(3, 4) finishes executing and is removed
    and multiply(3, 4) is added to the stack

    Stack:
    - multiply(3, 4)
    - do_math(3, 4)
    - main()

    then multiply(3, 4) finishes executing and is removed
    and divide(3, 4) is added to the stack

    Stack:
    - divide(3, 4)
    - do_math(3, 4)
    - main()

    then divide(3, 4) finishes executing and is removed
    then do_math(3, 4) finishes executing and is removed
        the value is assigned to the result variable
        which is Onwed by the main() function
    then the println! macro is called
    then the main() function finishes executing and is removed from the stack
*/

fn main() {
    let result = do_math(3, 4);
    println!("Result: {}", result);
}

fn do_math(a: i32, b: i32) -> i32 {
    let sum = add(a, b);
    let difference = subtract(a, b);
    let product = multiply(a, b);
    let quotient = divide(a, b);
    sum + difference + product + quotient
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a -b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> i32 {
    a / b
}

// The Heap
/*
    a less organized place for storing data compared to the stack.
    When data is stored on the heap, a certain amount of space is
    requested. The memory allocator finds a big enough space to hold
    the data and returns a pointer, which then is stored on the stack
    (because the pointer is of a known, fixed size). The pointer references
    the location in the heap where the data is stored.

    Data stored in the heap has a dynamic size and can grow or shrink
    at runtime. Accessing data from the heap is slower and data stored on
    the heap needs a pointer to be saved on the stack to access the data.
*/

// Pointers
/*
    a variable that is stored on the Stack and holds the memory address
    of a piece of data that is stored somewhere else (likely on the Heap).
*/

// Memory Allocation
/*
    the process of reserving a block of memory from the Heap
    and storing the pointer to it on the Stack.
*/

fn example() {
    let x = Box::new(5); // by default, scalars are stored on the stack, but can be stored in the Heap using the Box type
    println!("x = {}", x);
}

/*
Stack:             Heap:
_______________  _______________
|              |  |             |
| example()    |  |             |
| variable x --|--|--> value    |
|              |  |             |
---------------  ---------------
*/