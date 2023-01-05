use std::io;
use rand::Rng; 
use std::cmp::Ordering; 
pub fn main() {
    println!("guessing Game!!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("please input your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");
        
        println!("you guessed : {guess}");
    
        let mut guess: u32 = guess.trim().parse().expect("Please type a number!");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
           } 
       
    }
} 