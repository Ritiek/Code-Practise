fn remove_nb(m: i32) -> Vec<(i32, i32)> {
	let sum = (m*(m + 1))/2;
	for a in 1..m+1 {
		for b in 1..m+1 {
			if sum - (a + b) == a*b {
				return vec![(a, b), (b, a)];
			}
		}
	}
	vec![]
}

fn main() {
	let result = remove_nb(26);
	println!("{:?}", result);
}
