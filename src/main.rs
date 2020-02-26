use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("GUESSING GAME");

    let secret_number = rand::thread_rng().gen_range(1, 101); // 1-100

    // println!("The secret_number is: {}", secret_number);

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // convert user input into unsigned 32-bit number
    // simultaneously, we shadow `guess`, converting it from a string to a integer type
    // `trim()` removes carriage returns \n from user input
    // `:` annotates the variable tpye, meaning we define guess's type as a unsigned 32 bit number
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("Congrats! You win!"),
    }
}
