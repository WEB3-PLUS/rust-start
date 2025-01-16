use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("guess the number:");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("the secert number is :{secret_number}");

    loop {
        println!("please input your guess");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("faild to readline");

        println!("you guessed :{guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                return;
            }
        }
    }
}
