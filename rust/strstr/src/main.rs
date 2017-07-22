/* Check if a given string is a subset of another string.

 * If it is a subset, return the position of the given
 * subset string in another string

 * otherwise return -1

 * Taken from CodeFights.com
 */

fn strstr(s: &str, x: &str) -> i32 {
	let total_length = s.len();
	let piece_length = x.len();
	let total_range = 0..(total_length - piece_length);
	for piece_range in total_range {
		let test_piece = &s[piece_range..(piece_range + piece_length)];
		if test_piece == x {
			println!("{}", test_piece);
			return (piece_range + 1) as i32;
		}
	}
	-1
}

fn main() {
	let x = "IsAws";
	let s = "CodefightsIsAwesome";
	println!("{}", (strstr(s, x)));
}
