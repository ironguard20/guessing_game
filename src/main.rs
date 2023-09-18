use rand::Rng;
use std::cmp::Ordering;
use std::io; // Import io from the standard library and bring it into scope

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    loop {
        println!("\nPlease input your guess.");

        let mut guess = String::new(); // This line creates a mutable variable "guess" that is currently bound to a new, empty instance of a String. So guess = "" in this case.


        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => print!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
