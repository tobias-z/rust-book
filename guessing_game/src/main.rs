use std::{cmp::Ordering, io};

use rand::Rng;

fn get_input() -> String {
    println!("Please input your number");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    return guess;
}

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        let input = get_input();
        if input.trim() == "quit" {
            break;
        }
        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small");
            }
            Ordering::Greater => {
                println!("Too big");
            }
            Ordering::Equal => {
                println!("YOU WIN!");
                break;
            }
        }
    }
}
