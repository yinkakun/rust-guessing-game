use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number game 👀");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        println!("Guess a number 🐝");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Your guess isn't a number 🙄");
                continue;
            }
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The number you guessed is too small"),
            Ordering::Greater => println!("The number you guessed is too big"),
            Ordering::Equal => {
                println!("You win! 🤯");
                break;
            }
        };
    }
}
