use std::io; //std = standard library, io = io library
use std::cmp::Ordering::{Equal, Greater, Less};
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println! is a macro
    println!("========Guess the number!========");
    loop {
        println!("Please input your guess:");
        let mut guess = String::new(); //in rust variables are immutable by default. Use mut to turn it into a mutable variable :: =  calls for static methods
        
        io::stdin().read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Less => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
