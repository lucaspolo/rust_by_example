#![allow(dead_code)]

#[derive(Debug)]
struct Person {
	name: String,
	age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
	x: f32,
	y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
	top_left: Point,
	bottom_right: Point,
}

fn rect_area(rectangle: Rectangle) -> f32 {
	let Rectangle {
		top_left: top_left,
		bottom_right: bottom_right,
	} = rectangle;

	(bottom_right.x - top_left.x) * (top_left.y - bottom_right.y)
}

fn main() {
	let name = String::from("Peter");
	let age = 27;
	let peter = Person { name, age };

	println!("{:?}", peter);

	let point: Point = Point { x: 10.3, y: 0.4 };
	println!("point coordinates: ({}, {})", point.x, point.y);

	let bottom_right = Point { x: 5.2, ..point };

	// `botton_right.y` will be the same as ` point.y` because we used that field
	// from `point`
	println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

	// Destructure the point using a `let` binding
	let Point { x: left_edge, y: top_edge } = point;
	println!("{} {}", left_edge, top_edge);

	let _rectangle = Rectangle {
		// structure instantiation is an expression too
		top_left: Point { x: left_edge, y: top_edge },
		bottom_right: bottom_right,	
	};

	let _unit = Unit;

	let pair = Pair(1, 0.1);

	println!("pair contains {:?} and {:?}", pair.0, pair.1);

	let Pair(integer, decimal) = pair;
	println!("pair contains {:?} and {:?}", integer, decimal);

	println!("Rectangle area {}", rect_area(_rectangle));
}
