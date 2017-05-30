use std::f64::consts::PI;

fn square_area_to_circle(size: f64) -> f64 {
	let radius =  size.sqrt() / 2.0;
	radius * radius * PI
}

fn main() {
	let result = square_area_to_circle(20.0);
	println!("{}", result);
}
