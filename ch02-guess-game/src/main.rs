use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Processing a Guess
    println!("Guess the number!");

    let mut r = rand::thread_rng();
    let secret_number: u32 = r.gen_range(1, 10);

    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // compare the number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
