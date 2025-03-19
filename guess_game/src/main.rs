use std::{io, thread::Thread};
use rand::Rng;
use anyhow::{Result, Context};

fn main() -> Result<()>{
    println!("Guess the number!");
    let mut rng= rand::rng();
    let secret:u32 = rng.random_range(0..100);

    let mut check = false;
    let mut guess = String::new();
    let mut num: u32;

    while !check{
        println!("Please input your guess.");
        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .context("Failed main function")?;
        num = guess.trim().parse::<u32>().context("Failed to convert string")?;
        check = compare_guess(&secret, &num);
    }
    
    Ok(())
}


fn compare_guess(secret: &u32, guess: &u32) -> bool{
    if guess > secret{
        println!("Guess is higher");
        false
    }
    else if guess < secret {
        println!("Guess is lower");
        false
    }
    else {
        println!("Bingo!!!");
        true
    }
}