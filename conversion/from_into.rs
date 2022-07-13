use std::convert::From;

#[derive(Debug)]
struct Number {
	value: i32,
}

impl From<i32> for Number {
	fn from(item: i32) -> Self {
		Number { value: item }
	}
}

fn main() {
	let my_str = "hello";
	let my_string = String::from(my_str);

	let num = Number::from(30);
	println!("My number is {:?}", num);

	// Into trait is the From reciprocal trait
	let int = 5;
	let num_2: Number = int.into();
	println!("My second number is {:?}", num_2);
}
