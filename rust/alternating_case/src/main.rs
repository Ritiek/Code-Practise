fn to_alternating_case(s: &str) -> String {
	let mut collection = String::new();
	for c in s.chars() {
		if c.is_uppercase() {
			collection = format!("{}{}", collection, c.to_lowercase());
		} else if c.is_lowercase() {
			collection = format!("{}{}", collection, c.to_uppercase());
		} else {
			collection = format!("{}{}", collection, c);
		}
	}
	collection
}

fn main() {
	let result = to_alternating_case(&"hello world");
	println!("{}", result);
}
