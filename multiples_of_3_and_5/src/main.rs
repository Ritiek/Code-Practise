fn multiply(num: i32) -> i32 {
	let mut holder: Vec<i32> = vec![];
	for n in 1..num {
		if n % 3 == 0 && n % 5 == 0 ||
           n % 3 == 0 ||
           n % 5 == 0 {
			holder.push(n);
		}
	}
	holder.iter().sum()
}

fn main() {
	let result = multiply(10);
	println!("{}", result);
}
