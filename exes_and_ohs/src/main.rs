fn xo(string: &str) -> bool {
	string.to_lowercase().matches("x").count() == string.to_lowercase().matches("o").count()
}

fn main() {
	let result = xo("ooxx");
	println!("{}", result)
}
