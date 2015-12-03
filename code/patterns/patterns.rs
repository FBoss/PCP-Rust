fn main() {


	//any Pattern
	let x1 = 6;

	match x1 {
	1 => println!("one"),
	2 => println!("two"),
	3 => println!("three"),
	_ => println!("anything"),

	}
	//-------------------------------------------
	println!("-------------------------------------------");

	//multiple Pattern
	let x2 = 1;

	match x2 {
			1 | 2 => println!("one or two"),
			3 => println!("three"),
			_ => println!("anything"),
		}

	//-------------------------------------------
	println!("-------------------------------------------");

	//Destructuring

	struct Point0 {
	x: i32,
	y: i32,
	}

	let origin = Point0 { x: 0, y: 0 };

	match origin {
			Point0 { x, y } => println!("({},{})", x, y),
		}

	struct Point1 {
	x: i32,
	y: i32,
	}


	let origin = Point1 { x: 0, y: 0 };

	match origin {
			Point1 { x: x1, y: y1 } => println!("({},{})", x1, y1),
		}

	struct Point2 {
	x: i32,
	y: i32,
	}

	let origin = Point2 { x: 0, y: 0 };

	match origin {
			Point2 { x, .. } => println!("x is {}", x),
		}
	//-------------------------------------------
	println!("-------------------------------------------");

	//ref

	let r1 = 5;

	match r1 {
			ref r => println!("Got a reference to {}", r),
		}

	let mut r2 = 5;

	match r2 {
			ref mut mr => println!("Got a mutable reference to {}", mr),
		}
	//-------------------------------------------
	println!("-------------------------------------------");

	//ranges

	let range1 = 1;

	match range1 {
			1 ... 5 => println!("one through five"),
			_ => println!("anything"),
		}

	let range2 = 'd';

	match range2 {
			'a' ... 'j' => println!("early letter"),
			'k' ... 'z' => println!("late letter"),
			_ => println!("something else"),
		}

	//-------------------------------------------
	println!("-------------------------------------------");

	//bindings

	let bind1 = 3;

	match bind1 {
			e @ 1 ... 5 => println!("got a range element {}", e),
			_ => println!("anything"),
		}

	//-------------------------------------------
	println!("-------------------------------------------");

	//guards

	enum OptionalInt {
	Value(i32),
	Missing,
	}

	let guard = OptionalInt::Value(5);

	match guard {
			OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
			OptionalInt::Value(..) => println!("Got an int!"),
			OptionalInt::Missing => println!("No such luck."),
		}
}