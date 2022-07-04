use std::fmt;

fn main() {
	// Generic print
	println!("{} days", 31);

	// Named arguments
	println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brow fox", verb="jumps over");

	//Different formatting can invoked by specified format character after a `:`
	println!("Base 10 repr:               {}",   69420);	
	println!("Base 2 (binary) repr:       {:b}", 69420);	
	println!("Base 8 (octal) repr:        {:o}", 69420);	
	println!("Base 16 (hexadecimal) repr: {:x}", 69420);	
	println!("Base 16 (hexadecimal) repr: {:X}", 69420);	

	// Right-align text
	println!("{number:>5}", number=1);
	// Same with named arguments, will printing with 0-left too.
	println!("{number:0>width$}", number=1, width=5);

	// Rust checks the correct number of arguments
	// println!("My name is {0}, {1} {0}", "Bond");
	println!("My name is {0}, {1} {0}", "Bond", "James");

	#[allow(dead_code)]
	struct Structure(i32);

	// Not compile because Structure does not implement fmt::Display trait
	// println!("This struct `{}` won't print...", Structure(3));


	impl fmt::Display for Structure {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			write!(f, "{}", self.0)
		}
	}

	println!("This struct `{}` will print after implements the Display trait", Structure(3));

	// After Rust 1.58 you can directly get variables
	let number: f64 = 1.0;
	let width: usize = 6;
	println!("{number:>width$}");
}
