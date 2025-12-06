use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn initialize() -> i32 {
    println!("Guess the number!");
    get_random_number()
}

pub fn run_main_loop(secret_number: i32) {
    loop {
        let number_guess: i32 = read_user_input();
        match number_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Yes! You have entered: {number_guess}");
                break;
            },
        }
    }
}

fn get_random_number() -> i32 {
    rand::rng().random_range(1..=100)
}

fn read_user_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            read_user_input()
        }
    }
}
