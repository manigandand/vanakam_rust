use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Vanakam 🙏 Rust!");
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        let win = guess_number(secret_number);
        if win {
            break
        }
    }
}

fn guess_number(secret_number: u32) -> bool {
    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed: {}", guess);
    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too small 😔");
            return false;
        }
        Ordering::Greater => {
            println!("Too big 😬");
            return false;
        }
        Ordering::Equal => {
            println!("Congrats, You win 🥳 🎉");
            return true;
        }
    }
}