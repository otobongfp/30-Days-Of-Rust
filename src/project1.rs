use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn guessing_game(){
    //I have added useful to make the code clearer
    //If certain points are not clear, please refer to the rustlang book
    //mentioned on the readme

    //printing output to the terminal
    println!("\n\t ____A GUESSING GAME____");
    println!("Please before continuing, what is your name");

    //mut means the variable can change
    let mut name = String::new();

    //To read the input from the terminal we use the io lib
    io::stdin()
        .read_line(&mut name)
        .expect("Something went wrong with the input");

    println!("\n ☺️ Welcome, {name} I hope to see you get Rusty! for the next couple of days.\n Let's dive in!!!");


    println!("Now Guess a Number between 1 and 5 (1 to 5)");

    //Random Number generation using the rand::Rng crate

    let secret_number = rand::thread_rng().gen_range(1..5);

    //After creating a secret number we want to be able to allow a user to try their attempts

    loop{
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Someting went wrong while reading the guess!");

        //conver the guess string to a number to make it easier to compare
        //with the secret number

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Your Guess is less than the correct thing"),
            Ordering::Greater => println!("Your Guess is Greater than the secret number"),
            Ordering::Equal => {println!("YOU GOT IT, {name}"); break;}
        }
    }
    
}