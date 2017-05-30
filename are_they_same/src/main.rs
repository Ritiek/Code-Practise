fn comp(mut a: Vec<i64>, mut b: Vec<i64>) -> bool {
	a = a.iter().map(|x| x.pow(2)).collect();
	a.sort();
	b.sort();
	a == b
}

fn main() {
	let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
	let a2 = vec![11*11, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
	let result = comp(a1, a2);
	println!("{}", result);
}
