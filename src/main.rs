use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Let's play \"Guess the number\"");

    let secret_number = rand::thread_rng().gen_range(1..101);
    //Or .gen_range(1..=100)

    loop{
        println!("Please type your guess.");
        
        let mut guess = String::new(); //creating mutable variable

        io::stdin() //user input
        .read_line(&mut guess) //bind the input to variable reference
        .expect("Failed to read line"); //anticipate error

        let guess: u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess); //print the result

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You got it! Hooray!");
                break;
            },
        }
    }

    
    

}
