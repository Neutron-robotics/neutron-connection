use std::io;

fn main() {
    println!("Hello, world!");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Fails to read new line");
    
    println!("You guessed: {guess}");
}