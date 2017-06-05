fn duplicate_encode(word: &str) -> String {
	let mut encoded = String::new();
	let new_word =  word.to_lowercase();
	for char in new_word.chars() {
		let count = new_word.matches(char).count();
		if count == 1 {
			encoded += "("
		} else {
			encoded += ")"
		}
	}
	encoded
}

fn main() {
	let result = duplicate_encode("recede");
	println!("{}", result);
}
