use std::io;
use std::cmp::Ordering;
mod guess;

fn main() {
    let have = vec!["guess"];
    println!("What would you like to play? I have {:?}", have);
    loop {
        let mut game = String::new();
        io::stdin()
            .read_line(&mut game)
            .expect("something did not work");
        game = game.trim().parse().unwrap();
        if game == "guess" {
            guess::main();
            break;
        }else { 
            println!("I do not know that game sorry. I have {:?}", have);
        }
    }
}
