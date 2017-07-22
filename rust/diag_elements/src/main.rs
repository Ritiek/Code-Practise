fn main() {
    let matrix = vec![[1,   2,  3,  4],
                      [5,   6,  7,  8],
                      [9,  10, 11, 12],
                      [13, 14, 15, 16]];

	for i in 0..matrix.len() {
		println!("{}", matrix[i][i]);
	}
}
