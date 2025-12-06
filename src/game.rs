use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn initialize() -> i32 {
    println!("Guess the number!");
    get_random_number()
}

pub fn run_main_loop(secret_number: i32) {
    loop {
        let mut number_guess: i32 = read_user_input();
        match number_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Yes! You have entered: {}", number_guess);
                break;
            },
        }
    }
}

fn get_random_number() -> i32 {
    let secret_number = rand::rng().random_range(1..=100);
    secret_number
}

fn read_user_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please type a number!")
}

fn evaluate_guess(guess: i32, secret: i32) {
    if guess < 0 {
        println!("Please enter a valid number between 1 and 20.");
    } else if guess < secret {
        println!("Too small!");
    } else if guess > secret {
        println!("Too big!");
    }
}
