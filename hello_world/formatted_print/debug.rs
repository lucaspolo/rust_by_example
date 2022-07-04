// struct UnPrintable(i32);
// 
// // This `derive` attribute automatically creates the implementation
// // required to make this `struct` printable with `fmt::Debug`.
// #[derive(Debug)]
// struct DebugPrintable(i32);

// All the std library types are automatically printable with {:?}

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
	println!("{:?} months in a year.", 12);
	println!("{1:?} {0:?} is the {actor:?} name.",
		     "Slater",
		     "Christian",
		     actor="actor's");

	// `Structure` is printable!
	println!("Now {:?} will print!", Structure(3));

	// The problem with `derive` is there is not control over how
	// the results look. What if I want this to just show a `7`?
	println!("Now {:?} will print!", Deep(Structure(7)));

	#[derive(Debug)]
	struct Person<'a> {
		name: &'a str,
		age: u8
	}

	let name = "Peter";
	let age = 27;
	let peter = Person { name, age };

	println!("{:#?}", peter);
}
