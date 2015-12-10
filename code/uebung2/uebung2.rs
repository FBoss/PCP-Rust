fn search_int(f: Vec<u32>){
	for x in f.iter() {
		match *x {
				1 => println!("one found"),
				3 => println!("three found"),
				_ => {},
			}
	}

	println!("---------------------------------------------------");
}

fn search_char(f: Vec<char>){
	for x in f.iter() {
		match *x {
				'a' ... 'm' => println!("letter_front"),
				'n' ... 'z' => println!("letter_back"),
				_ => {},
			}
	}

	println!("---------------------------------------------------");

}

fn search_string(f: Vec<&str>){
	for x in f.iter() {
		match *x {
				"ccds" => println!("String found"),
				_ => {},
			}
	}
	println!("---------------------------------------------------");
}

fn main() {
	let vec1 = vec![1, 2, 3, 4];
	let vec2 = vec!['a', 'b','c','d','q','p','z'];
	let vec3 = vec!["aaa", "bbb","ccds","ccas"];

	search_int(vec1);
	search_char(vec2);
	search_string(vec3);
}