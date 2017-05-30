fn square_sum(vec: Vec<i32>) -> i32 {
	vec.iter().map(|x| x.pow(2)).sum()
}

fn main() {
	let result = square_sum(vec![1, 2, 2]);
	println!("{}", result);
}
