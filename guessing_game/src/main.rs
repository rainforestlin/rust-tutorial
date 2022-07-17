use std::io;

use rand::Rng;

fn main() {
    println!("Guessing a number");
    println!("Input your guess");
    let secret_num = rand::thread_rng().gen_range(1..=500);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("your guess {guess}");
        let guess: u32 = guess.trim().parse().expect("please type a number");
        match guess.cmp(&secret_num) {
            std::cmp::Ordering::Equal => {
                println!("you win ");
                break;
            }
            std::cmp::Ordering::Greater => println!("too big"),
            std::cmp::Ordering::Less => println!("too small"),
        }
    }
}
