/* Given a sequence of integers as an array, determine whether
 * it is possible to obtain a strictly increasing sequence by
 * removing no more than one element from the array.

 * Example:

 * For sequence = [1, 3, 2, 1], the output should be
 * almost_increasing_sequence(sequence) = false

 * There is no one element in this array that can be removed in order to get a strictly increasing sequence.

 * For sequence = [1, 3, 2], the output should be
 * almost_increasing_sequence(sequence) = true

 * You can remove 3 from the array to get the strictly increasing sequence [1, 2]. Alternately, you can remove 2 to get the strictly increasing sequence [1, 3].

 * Originally appeared in Arcade Mode in CodeFights.com
 */

fn almost_increasing_sequence(sequence: Vec<i32>) -> bool {
	for x in 0..(sequence.len()) {

		let mut original = sequence.to_vec();
		original.remove(x);

		let unsorted_vector = original.to_vec();
		original.sort();

		if unsorted_vector == original {
			return true;
		}
	}
	false
}

fn main() {
	let sequence = vec![1, 3, 2, 1];
	println!("{}", almost_increasing_sequence(sequence));
}
