enum WebEvent {
	PageLoad,
	PageUnload,
	// like tuple structs,
	KeyPress(char),
	Paste(String),
	Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
	match event {
		WebEvent::PageLoad => println!("page loaded"),
		WebEvent::PageUnload => println!("page unloaded"),
		// Destructure `c` from inside the `enum`
		WebEvent::KeyPress(c) => println!("pressed '{}'", c),
		WebEvent::Paste(s) => println!("pasted \"{}\"", s),
		// Destructure `Click` into `x` and `y`
		WebEvent::Click { x, y } => {
			println!("click at x={}, y={}", x, y);
		},
	}
}

enum VeryVerboseEnumOfThingsToDoWithNumber {
	Add,
	Subtract,
}

// creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumber;

impl VeryVerboseEnumOfThingsToDoWithNumber {
	fn run(&self, x:i32, y: i32) -> i32 {
		match self {
			// Self acts like a alias
			Self::Add => x + y,
			Self::Subtract => x - y,
		}
	}
}

fn main() {
	let pressed = WebEvent::KeyPress('x');
	// `to_owned()` creates an owned `String` from a string slice
	let pasted  = WebEvent::Paste("my text".to_owned());
	let click   = WebEvent::Click { x: 20, y: 80 };
	let load    = WebEvent::PageLoad;
	let unload  = WebEvent::PageUnload;

	inspect(pressed);
	inspect(pasted);
	inspect(click);
	inspect(load);
	inspect(unload);

	let x = Operations::Add;
	println!("{}", x.run(11, 13));
}