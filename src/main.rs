use std::io; //the io library comes from the standard library, known as std

fn main() {
    println!("Guess the number!"); //prints a string to screen

    println!("Please input your guess"); //prints a string to screen

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
