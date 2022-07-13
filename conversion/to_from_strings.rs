use std::fmt;

struct Circle {
	radius: i32
}


// Rather than implements the ToString trait, 
// we implement Display that automatically provides ToString
impl fmt::Display for Circle {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Circle of radius {}", self.radius)
	}
}

fn main() {
	let circle = Circle { radius: 6 };
	println!("{}", circle.to_string());

	// parsing string to number
	let parsed: i32 = "5".parse().unwrap();
	let turbo_parsed = "10".parse::<i32>().unwrap();

	let sum = parsed + turbo_parsed;
	println!("Sum: {:?}", sum);
}
