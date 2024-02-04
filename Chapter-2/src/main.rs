use std::io;
// using Rand crate, an external crate specified as a dependency in Cargo.toml
// the Rng trait defines nethods that random number generators implement
use rand::Rng;
use std::cmp::Ordering; // Ordering is another enum type (Less, Greater, Equal)

fn main() {
    println!("Guess the number!");

    // thread_rng: a random number generator local to the current thread of execution and is seeded by the operating system
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // variables definded in Rust are immutable by default, have to specify "mut"
        let mut guess = String::new(); // mutable

        // if we didn't import std::io, could still call the below function through std::io::stdin
        io::stdin()
            // we are passing in a reference to guess, and references are immutable by default too so need to specify "mut"
            .read_line(&mut guess) // read_line returns Result enum type (Ok or Err)
            .expect("Failed to read line"); // expect will cause program to crash if Result is Err

        // already defined guess, but Rust allows us to shadow the previous value of guess with a new one
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // refine behavior to ignore non-number and continue to next loop instead of crashing program
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
