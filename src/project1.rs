use std::io;

pub fn counter(){
    //This is how Rust prints to the cli
    println!("\nINPUT A NUMBER TO COUNTDOWN FROM:");

    let mut value = String::new();

    //this is from the standard library for handling inputs
    io::stdin()
        .read_line(&mut value)
        .expect("Here is the input");
    
    //convert to uint
    let mut value:u32 = value.trim().parse().expect("Could not convert");

    loop{
        if value > 0{
            println!("{}", value);
            value -= 1;
        }else{
            break;
        }   
    }
}