use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main()
{
    let secret_number: i8 = rand::thread_rng().gen_range(0..9);
    let max_guess: i8 = 9;
    let min_guess: i8 = 0;

    loop
    {
        println!("Guess a number (0-9)!");

        let mut guess: String = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");
    
        let guess: i8 = match guess.trim().parse()
        {
            Ok(guess) => guess,
            Err(_) => 
            {
                println!("Invalid input");
                continue;
            }
        };

        if guess > max_guess
        {
                println!("Invalid input");
                continue;
        }
        if guess < min_guess
        {
                println!("Invalid input");
                continue;
        }
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("You lose :("),
            Ordering::Greater => println!("You lose :("),
            Ordering::Equal => 
            {
                println!("You win :D");
                break;
            }
        }
    }
}