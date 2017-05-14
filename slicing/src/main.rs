fn almost(vector: &mut Vec<i32>) {
	let original = vector.to_vec();
	println!("{:?}", vector);
	for x in 0..(vector.len()) {
		vector.remove(x);
		if vector == vector.sort() {
			println!("true");
		}
		let mut vector = original;
	}
	println!("false");
}

fn main() {
	let mut vector = vec![1, 2, 3, 4, 5, 6];
	almost(&mut vector);
}
