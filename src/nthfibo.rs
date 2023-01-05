use std::io;
use colored::*;

pub fn main() {
  println!("nth Fibonacci Number");
    println!("Enter a value for 'n'");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read value 'n'");
    let n: u32 = n.trim().parse().expect("Please enter a number");

    let mut  a = 0;
    let mut b = 1;
    let mut c = a + b;

    if n == 1 {
        println!(
            "{}th Fibonacci number is: {}", 
            n, 
            a.to_string().bold().italic().green()
        );
    }
    else if n == 2 {
        println!(
            "{}th Fibonacci number is: {}", 
            n, 
            b.to_string().bold().italic().green()
        );
    } 
    else if n == 3 {
        println!(
            "{}th Fibonacci number is: {}", 
            n, 
            c.to_string().bold().italic().green()
        );
    } 
    else {
        let mut count = 3;
        while count < n {  
            a = b;
            b = c;
            c = a + b; 
            count += 1;          
        }
        println!(
            "{}th Fibonacci number is: {}", 
            n, 
            c.to_string().bold().italic().green()
        );
    }
}