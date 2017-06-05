fn trotter(n: i32) -> i32 {
	if n == 0 {
		return -1;
	}
	let mut collector: Vec<char> = Vec::new();
	let mut multiplier = 0;

	while collector.len() < 10 {
		multiplier += 1;
		let string = (n*multiplier).to_string();
		for char in string.chars() {
			collector.push(char);
		}
		collector.sort();
		collector.dedup();
	}
	n*multiplier
}

fn main() {
	let result = trotter(0);
	println!("{}", result);
}
