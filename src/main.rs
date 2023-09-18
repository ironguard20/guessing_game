use std::io; // Import io from the standard library and bring it into scope
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // This line creates a mutable variable "guess" that is currently bound to a new, empty instance of a String. So guess = "" in this case.


    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

}
