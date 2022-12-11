#![allow(unused)]
use std::cmp::Ordering;
use std::io;
use std::sync::BarrierWaitResult;
use rand::Rng;

fn main() {
    //1 generate the secret number
    let secret: i32 = rand::thread_rng().gen_range(1..101);

    //2 ask user number
    println!("Guess the number: (hint {}):", secret);

    loop {
        //3 read user number as a string
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //4 verify user number and tell user
        let guess: i32 = guess.trim().parse().expect("failed, plese type a number");
        println!("Your guess: {}", guess);
        match guess.cmp(&secret){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal =>
            {
            println!("You win!!");
            break;
            } 
        }  
    }
    
    println!("\nGoodbye\n");
    
}


