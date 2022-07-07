fn main() {
	let long_lived_binding = 1;

	{
		// This binding only exists in this block
		let short_lived_binding = 2;

		println!("inner short: {}", short_lived_binding);

		let long_lived_binding = 5;
		println!("shadowed value: {}", long_lived_binding);
	}

	// println!("outer short: {}", short_lived_binding);
	println!("outer long: {}", long_lived_binding);
}
