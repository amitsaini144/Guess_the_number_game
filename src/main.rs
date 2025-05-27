use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number Game!!!");

    let answer = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Please Enter your guess between 1 and 10:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Unable to read!");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&answer) {
            Ordering::Less => println!("Too Low!"),
            Ordering::Equal => {
                println!("Perfect You win!");
                break;
            }
            Ordering::Greater => println!("Too High!"),
        }
    }
}
