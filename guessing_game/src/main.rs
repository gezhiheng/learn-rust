use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut start_number = 1;
    let mut end_number = 100;

    println!("Please enter your guess.");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter number");
                continue;
            },
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                if guess < start_number {
                    println!("out of range");
                } else {
                    println!("Too small");
                    start_number = guess;
                    println!("{start_number} - {end_number}");
                }
            },
            Ordering::Greater => {
                if guess > end_number {
                    println!("out of range");
                } else {
                    println!("Too big");
                    end_number = guess;
                    println!("{start_number} - {end_number}");
                }
            },
            Ordering::Equal => {
                println!("you win");
                break;
            },
        }
    }
}
