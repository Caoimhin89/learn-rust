fn main() {
    which_is_bigger(3, 5);
    which_is_bigger(10, 2);
    which_is_bigger(3, 3);

    // using if in a let statement
    let condition = true;
    let number = if condition {
        10
    } else {
        100
    }; // note that types must be the same
    println!("The value of number is: {}", number);

    println!("{}", cead_mile_failte());
    skip_evens();
    loop_label();
    while_loop();
    for_loop();
    find_match(3);
    find_match(10);
    find_match_with_guard(1);
    find_match_with_guard(3);
    find_match_with_guard(4);
    find_match_with_guard(-1);

    let state = State::Idle;
    print_state(state);
}

// If expressions
fn which_is_bigger(a: i32, b: i32) -> i32 {
    if a < b {
        println!("{} is less than {}", a, b);
        return a;
    } else if b < a {
        println!("{} is greater than {}", a, b);
        return b;
    } else {
        println!("{} is equal to {}", a, b);
        return 0;
    }
}


// Loops
/*
    - loop
    - while
    - for
*/

fn cead_mile_failte() -> String {
    let mut counter = 0;
    
    loop {
        println!("{}. Failte!", counter + 1);

        counter += 1;

        if counter == 100_000 {
            break ("Cead mile failte!").to_string(); // we can return values from a loop
        }
    }
}

fn skip_evens() {
    let mut counter = 0;

    loop {
        counter += 1;

        if counter % 2 == 0 {
            continue; // skip to next iteration
        }

        if counter > 9 {
            break;
        }

        println!("The number is: {}", counter);
    }
}

fn loop_label() {
    let mut outer_counter = 0;
    'outer: loop {
        let mut inner_counter = 0;
        'inner: loop {
            println!(
                "Outer loop: {}, Inner loop: {}",
                outer_counter, inner_counter
            );
            if inner_counter == 5 {
                break 'inner; // exits inner loop
            }
            if outer_counter == 2 {
                break 'outer; // exits outer loop
            }
            inner_counter += 1;
        }
        outer_counter += 1;
    }
}

fn while_loop() {
    let mut counter = 0;

    while counter < 10 {
        println!("{}. Hello, world!", counter + 1);
        counter += 1;
    }
}

fn for_loop() {
    let numbers = [1, 2, 3, 4, 5];

    for number in numbers.iter() {
        println!("The number is: {}", number);
    }

    // or use a range
    for num in 1..5 { // 1 to 5, not inclusive
        println!("The number is: {}", num);
    }

    for num in 1..=5 { // 1 to 5, inclusive
        println!("The number is: {}", num);
    }
}


// Pattern matching
/*
    - match
    - if let
*/

fn find_match(x: i32) {
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other"),
    }
}

// ARMS
/*
Arms are the individual clauses in a match expression.
Each arm consists of a few parts:
- pattern,
- expression,
- a Guard (optional).

Pattern: The pattern is used to match the value of
the expression. If the pattern matches the value of
the expression, the corresponding expression is
executed.

Expression: The expression will be returned as the
result of the match expression.

Guard (optional): The guard is an additional 
condition that must be satisfied for the arm to match.
*/

fn find_match_with_guard(x: i32) {
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        n if n > 3 => println!("Greater than three"),
        _ => println!("Other"),
    }
}

enum State {
    Idle,
    Running,
    Waiting,
}

fn print_state(state: State) {
    if let State::Idle = state {
        println!("The system is idle");
    }
}