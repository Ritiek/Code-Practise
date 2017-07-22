//use std::io;

// use this to flush stdout (not sure why)
use std::io::{self, Write};

fn main() {
	print!("Enter some number: ");

	// flush stdout to print above code
	io::stdout().flush()
		.ok();

	// take input
	let mut input = String::new();

	// read input from.stdin
	io::stdin().read_line(&mut input)
		.ok();

	// convert number to int
	let trimmed = input.trim();
	match trimmed.parse::<u32>() {
		Ok(i) => {
			// number(factorial)
			let mut product = 1;
			for n in 1..(i+1) {
				product *= n;
			}
			println!("{0}! = {1}", i, product);
		}
		// catch exception; if not input == number
		Err(..) => println!("this was not an integer: {}", trimmed),
	}
}
