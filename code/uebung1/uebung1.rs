fn main() {
	let vec1 = vec![1, 2, 3, 4];
	for x in vec1.iter() {
		match *x {
				1 => println!("one found"),
				3 => println!("three found"),
				_ => {},
			}
	}

	println!("______________________________________________________");

	let vec2 = vec!['a', 'b','c','d','q','p','z'];
	for x in vec2.iter() {
		match *x {
				'a' ... 'm' => println!("letter_front"),
				'n' ... 'z' => println!("letter_back"),
				_ => {},
			}
	}

	println!("______________________________________________________");

	let vec3 = vec!["aaa", "bbb","ccds","ccas"];
	for x in vec3.iter() {
		match *x {
				"ccds" => println!("String found"),
				_ => {},
			}
	}
}