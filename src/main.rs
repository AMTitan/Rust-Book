use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("what is your number");
    let num = rand::thread_rng().gen_range(1..101);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("something did not work");
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        match guess.cmp(&num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
