use std::io::{self, Write};
use std::process;

fn main() {
	print!("Enter some number: ");
	io::stdout().flush().ok();

	let mut input = String::new();
	io::stdin().read_line(&mut input)
		.ok();

	let trimmed = input.trim();
	match trimmed.parse::<u32>() {
		Ok(i) => {
			for n in 2..(i) {
				if i % n == 0 {
					println!("{0} ain't prime", i);
					process::exit(1);
				}
			}
			println!("{0} is prime", i)
		}
		Err(..) => println!("integer pls"),
	}
}
