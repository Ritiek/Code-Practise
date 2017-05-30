fn dont_give_me_five(start: isize, end: isize) -> isize {
	let mut total = end - start + 1;
	for x in start..end {
		if x.to_string().contains("5") {
			total -= 1;
		}
	}
	total
}

fn main() {
	let result = dont_give_me_five(1, 9);
	println!("{}", result);
}
