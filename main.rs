extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guess the number!");

    let mut secret_number = rand::thread_rng().gen_range(1, 101);

let mut  trys: i32=0;
loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("failed to read line");
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed: {}", guess);
    trys=trys+1;
    println!("Your total guesses= {} ",trys);
    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   =>  {
                println!("You win!");
		println!("Do you want to play again? 1=yes 0=no");
		let mut option = String::new();
		    io::stdin().read_line(&mut option)
        .expect("failed to read line");
		let option : u32 = option.trim().parse()
		     .expect("Do you want to play again?");
		     
		if option == 0 {
		println!("GoodBye!");
                break;
		}
		else {
		secret_number = rand::thread_rng().gen_range(1, 101);
		trys=0;
		}
		
            }
    }
}
}
