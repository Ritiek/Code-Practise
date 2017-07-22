use std::io;
use std::process;

fn main() {
	let mut x = String::new();
	io::stdin().read_line(&mut x)
		.ok();
	let x: u32 = match x.trim().parse() {
		Ok(i) => i,
		Err(..) => process::exit(1),
	};

	let mut y = String::new();
	io::stdin().read_line(&mut y)
		.ok();
	let y: u32 = match y.trim().parse() {
		Ok(i) => i,
		Err(..) => process::exit(1),
	};

	println!("{}", x + y);
}
