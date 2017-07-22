fn add(args: &[i64]) -> i64 {
	args.iter().enumerate()
        .map(|(i, item)| ((i as i64) + 1)*item)
        .sum()
}

fn main() {
	let result = add(&[3, 4, 5]);
	println!("{}", result);
}
