use std::io;
use rand::Rng;

const MAX_ATTEMPTS: u32 = 5;

fn main() {
    println!("Number guessing game");
    let mut user_score = 0;
    let mut player_ready = true;


    loop {
        user_score += start_game_loop(generate_random_number());

        player_ready = ask_play_again();

        if player_ready != true {
            println!("Final score: {}", user_score);
            break;
        }
    }
}

/// generates a random number from 1 to 100, inclusive
fn generate_random_number() -> u32 {
    rand::rng().random_range(1..=100)
}

fn read_user_input() -> u32 {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    user_input.trim().parse().expect("Please type a number!")
}

fn compare_numbers(user_input: u32, random_number: u32) -> String {
    if user_input > random_number {
        return String::from("Too high!");
    } else if user_input < random_number {
        return String::from("Too low!");
    } else {
        return String::from("You win!");
    }
}

fn start_game_loop(random_number: u32) -> u32 {
    let mut attempts = 0;

    loop {
        let user_input = read_user_input();
        
        let text = compare_numbers(user_input, random_number);
        println!("{}", text);

        attempts += 1;

        if user_input == random_number {
            return 1;
        }

        if attempts == MAX_ATTEMPTS {
            println!("You lose! The number was: {}", random_number);
            return 0;
        }
    }
}

fn ask_play_again() -> bool {
    let mut user_input = String::new();
    println!("Do you want to play again? [y/n]");
    
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let parsed_user_input = user_input.trim().to_lowercase();

    parsed_user_input == "y" || parsed_user_input == "yes"
}


