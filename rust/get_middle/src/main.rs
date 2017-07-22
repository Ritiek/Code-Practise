fn get_middle(s: &str) -> String {
	let chvec = s.chars().collect::<Vec<_>>();
	let middle = chvec.len() / 2;
	if chvec.len() % 2 == 0 {
		return format!("{}{}", chvec[middle-1], chvec[middle]);
	} else {
		return chvec[middle].to_string();
	}
}

fn main() {
	let result = get_middle("test");
    println!("{}", result);
}
