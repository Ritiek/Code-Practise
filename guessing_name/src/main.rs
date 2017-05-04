// 1st option
use std::io;

// 2nd option
//use std::io::stdin;

fn main() {
	println!("Guess the number!");
	println!("Please input your guess.");
	let mut guess = String::new();

	/* io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line"); */

	// 1st option
	io::stdin().read_line(&mut guess).expect("Failed to read line");

	// 2nd option
	//stdin().read_line(&mut guess).expect("Failed to read line");

	// Use below to work without imports
	//std::io::stdin().read_line(&mut guess).expect("Failed to read line");

	println!("You guessed: {}", guess);
}

