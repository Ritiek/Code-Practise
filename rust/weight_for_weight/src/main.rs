fn order_weight(s: &str) -> String {
	let int_list = s.split(" ");
	for (rank, item) in int_list.enumerate() {
		println!("{}", item.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>());
	}
	s.to_string()
}

fn main() {
	let result = order_weight("56 65 74 100 99 68 86 180 90");
	println!("{}", result);
}
