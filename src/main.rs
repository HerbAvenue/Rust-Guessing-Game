use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Rust Guessing Game!");
    
    
    let ran_num = rand::thread_rng().gen_range(1..11);

    loop {
        println!("Please enter a number: ");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Cannot read input.");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is: {}", guess);
        println!("The number was: {}", ran_num);

        match guess.cmp(&ran_num) {
            Ordering::Less => println!("This number is less than the answer."),
            Ordering::Greater => println!("This number is greater than the answer."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }    
            
        }

    }
}
