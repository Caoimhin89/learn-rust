// Ownership Rules
/*
    - Each value in Rust has an owner
    - There can only be one owner at a time
    - When the owner goes out of scope, the value will be dropped
*/

// Variable scope
fn variable_scope {
    let x = 5; // x is valid from this point
    {
        let y = 10; // y is valid from this point
        println!("The value of y is: {}", y);
    } // y goes out of scope here
    println!("The value of x is: {}", x);
} // x goes out of scope here

// Transferring Ownership (moving values)
/*
    In most programming languages, when you assign a value
    to a variable, you are copying the value to the variable if
    it's a primitive type like an integer or boolean and if it's
    an object or a reference type, you are copying the reference
    to that object.

    In Rust, when you assign a value to a variable, you are moving
    the value to the variable and if you re-assign that variable to
    another one, the value is moved to the new variable and the old 
    variable is no longer valid.
*/
fn transfer_ownership() {
    let x = String::from("Hello");
    let y = x;
    // println!("The value of x is: {}", x); // compiler error
    println!("The value of y is: {}", y);

    /*
        In the above code x is moved to y and x is no longer
        valid after that. This is because String is a complex
        type and it's stored on the heap. Moving it to another
        variable is more efficient than copying it.
     */
}

// Copy Types
/*
    Moving values from one variable to another is only true
    for complex types like String, Vec, etc. Types that 
    implement the Copy trait are copied instead of moved.

    Some types implement the Copy trait and these types can
    be efficiently copied by simply assigning them to another
    variable. This is because they are stored on the stack and
    taking a copy of them is extremely cheap, since their size
    is fixed and known at compile time.

    These types are:

    - All integer types (i32, u32, i64, u64, etc)
    - bool
    - char
    - Floating point types (f32, f64, etc)
    - Tuples that contain only types that implement the Copy trait
*/

fn copy_value() {
    let x = 42;
    let y = x;
    println!("The value of x is: {}", x); // No compiler error, because the value of x is copied, not movied
    println!("The value of y is: {}", y);
}

// Working with Ownership
/*
    If moving a value from one variable to another is
    undesirable, and instead it is desirable to keep the
    original value and use it in another place there are a
    few options.

    1) A clone of the value can be made
    2) The reference to the original value can be used

    Cloning:
        Cloning creates a deep copy of the value along with its
    nested values and store it in a new location in memory,
    which usually requires memory allocation on the heap.

    Copying:
        When a variable is assigned to another variable which
    holds a value of a type that implements Copy, a shallow
    copy of the value is made, and each value will be stored
    separately and independently on the stack.

*/

fn clone_value() {
    let x = String::from("Dia duit");
    let y = x.clone(); // value has been re-allocated (cloned)
    println!("x is {}", x);
    println("y is {}", y);

    /*
        In this example, both x and y are valid, because x
        is cloned and a new value is created on the heap and
        asigned to y. This way, it is possible to keep both the
        original value and use it in another place.

        Using clone() can sometimes be expensive, especially when
        working with large chunks of data, because the entire value
        is copied to a new location in memory. So it is advisable to
        only use clone() when necessary.
     */
}

fn working_with_funcs() {
    let s = String::from("Dia is Muire duit");
    takes_ownership(s);
    // println!("s is {}", s); // compiler error

    /*
        When a value is passed to a function, the value
        is moved to the recipient function and the function
        becomes the owner of that value. Therefore, it is no
        longer available in the context in which the recieving
        function is called.

        However, functions can also cede ownership back to the
        calling context via their return values. This will allow
        the variable in the parent context to be re-assumed and used.
     */

    let b = String::from("Conas ata tu?");
    let b = takes_ownership_and_gives_back(b);
    println!("b is {}", {});

    /*
        This approach of passing ownership back and forth is not
        a great approach, however. It's difficult to reason about
        and since only one function can hold ownership at a time,
        it also makes concurrency impossible.

        Cloning could be used to acheive concurrency, but that means
        data being stored multiple times on the heap, which is expensive.

        The most efficient way to pass values for other functions to read
        is by using references. Instead of ceding ownership from
        one function to another, the address of the value stored in memory
        (the reference) can be passed from one function to another.
        That way multiple functions can read the same data from the same
        place in memory without taking ownership and without duplicating
        data and storing it multiple times in memory.
     */

    let a = String::from("Ce chaoi a bhfuil tu");
    borrows_by_ref(&a);
    println!("a is {}", a); // can still use the value, because ownership is retained

    let mut z = String::from("Dia duit"); // note "mut"
    borrow_and_change(&mut z);
    println!("z is {}", z); // Dia duit, a chara
}

fn takes_ownership(s: String) {
    println!("s is {}", s);
}

fn takes_ownership_and_gives_back(s: String) -> String {
    println!("s is {}", s);
    s
}

fn borrows_by_ref(s: &String) {
    println!("s is {}", s);

    /*
        To borrow a value from the original value, the &
        symbol is used. This is called a "reference".

        There are two types of reference in Rust.
        
        The first is an "Immutable Reference".
        This means that the value that is borrowed cannot be changed
        by the borrower. Trying to change a value that is borrowed
        will cause a compiler error.

        The second is a "Mutable Reference". 
        If it is necessary to change the value that is borrowed,
        then a mutable reference must be used. A mutable reference
        is created with the &mut symbol instead of the & symbol.
     */
    
    // s.push_str(", a chara?"); // attempt to change value, causes compiler error
}

fn borrow_and_change(s: &mut String) {
    s.push_str(", a chara");
}

// Limitations of Mutable References
/*
    If a variable is declared as mutable, then there can
    only be one mutable reference to a value within a given
    scope.

    With mutable references, you can not use multiple mutable 
    references at the same time, and you can not use mutable 
    references with immutable references at the same time, 
    this is a core safety feature in Rust to ensure that the 
    value is not changed unexpectedly.
*/

// example function that will not compile
/*
fn demonstrate_mut_ref_limitation() {
    let mut s = String::from("Dia duit");

    let r1 = &mut s;
    let r2 = &mut s;

    concatenate(r1, r2);
}
*/

fn concatenate(s1: &mut String, s2: &mut String) -> {
    let result = s1.to_string() + s2;
    result
}

// Strings and Slices
/*
    The String type is a wrapper around a vector of unsigned
    8-byte integers (Vec<u8>) that represents a UTF-8 encoded
    string. The String type is a growable, heap-allocated data
    structure that allows you to store a sequence of UTF-8 encoded
    characters.

    A Slice is a reference to a sequence of elements in a collection.
    In this case, it is a reference to a sequence of bytes in the String
    type. The slice type is &str, which is a reference to a sequence
    of UTF-8 encoded characters.
*/

fn strings_and_slices() {
    let my_string = String::from("Foghlaim Rust");

    let i_learn = &my_string(0..8);
    let rust = &my_string(9..);

    println("I'm learning {}", rust);
}

fn test_ownership() {
    let mut text = String::from("good morning");

    let prefix = get_prefix(&text);

    text = format!("Hello {}", text);

    println!("The prefix is: {}", prefix);
    println!("The text is: {}", text);
}

fn get_prefix(t: &String) -> String {
    let characters = t.chars();

    for (idx, char) in characters.enumerate() {
        if char == ' ' {
            return t[0..idx].to_string();
        }
    }
    t[..].to_string()
}


