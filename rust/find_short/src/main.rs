fn find_short(s: &str) -> u32 {
	let mut arr = s.split(" ").collect::<Vec<&str>>();
	arr.sort_by_key(|x| x.len());
	arr[0].len() as u32
}

fn main() {
	let result = find_short("turns out random test cases are easier than writing out basic ones");
	println!("{}", result);
}
