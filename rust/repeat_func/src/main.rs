use std::iter;

fn main() {
	println!("{}", repeat_str("a", 4));
}

fn repeat_str(src: &str, count: usize) -> String {
	String::new();
	/* As of Rust 1.16 */
	//src.repeat(count)
	/* Before Rust 1.16 */
	iter::repeat(src).take(count).collect()
}
