fn main() {
	let _immutable_binding = 1;
	let mut mutable_binding = 1;

	println!("Beferoe mutation: {}", mutable_binding);

	// Ok
	mutable_binding += 1;

	println!("After mutation: {}", mutable_binding);

	// _immutable_binding += 1;
}
