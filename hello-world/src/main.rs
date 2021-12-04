use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1..101);
    // println!("secret number = {}", secret);

    loop {
        println!("your guess ->");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
