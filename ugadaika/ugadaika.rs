extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Guess a number game!");
	let secret_num = rand::thread_rng().gen_range(1, 101);
	loop {
		println!("Enter your guess: ");
		let mut guess = String::new();
		io::stdin().read_line(&mut guess).expect("Couldn't read anything");
		let guess : u32 = match guess.trim().parse()
		{
			Ok(r) => r,
			Err(_) => {
				println!("Please, enter a number!");
				continue;
			},
		};
		println!("Your guess is: {}", guess);
		match guess.cmp(&secret_num)
		{
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("Congratulations, you won!");
				break;
			}
		}
	}
}
