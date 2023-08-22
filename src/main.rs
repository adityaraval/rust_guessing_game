use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret_number = rand::thread_rng().gen_range(0..1000);

    loop {
        println!("Randmo generated number {}", secret_number);
        println!("Please guess a number:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read");

        let guess: i32 = guess.trim().parse().expect("Please type a number");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You have won");
                break;
            }
            Ordering::Greater => {
                println!("You have guessed too big")
            }
            Ordering::Less => println!("You have guessed too small"),
        }
    }
}
