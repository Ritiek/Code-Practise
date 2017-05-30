fn high_and_low(numbers: &str) -> String {
	// Convert &str to vector with integers
	let bar: Vec<i32> = numbers.split(" ")
                               .map(|x| x.parse().unwrap()).collect();
	let min = bar.iter().min()
                 .unwrap().to_string();
	let max = bar.iter().max()
                 .unwrap().to_string();

	format!("{} {}", max, min)
}

fn main() {
	let result = high_and_low("1 2 3 4 5");
	println!("{}", result);
}
