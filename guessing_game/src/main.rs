// from the standard library (std) we want to use the io (input/output) library
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // using "!" indicate that this is a macro, non a normal function
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
    
        println!("Please input your guess.");

        // let apples = 5; // immutable
        // let mut bananas = 5; // mutable
        // ::new indicate that new is an "associated function" of the String type
        let mut guess = String::new();

        // we could have avoid the import and use: std::io::stdin
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        // println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // exit the loop
            } 
        }
    }
}
