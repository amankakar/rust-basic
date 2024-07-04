use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);


    loop {
        println!("enter your number..");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Yor guess is Less"),
            Ordering::Equal => {
                println!("You Guess right Number");
                break;
            }
            Ordering::Greater => println!("You guess Greater Number"),
        }
    }
}
