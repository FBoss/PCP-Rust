use std::io;

fn main() {

	println!("Please input an Integer form 1 to 10.");
	let reader: io::Stdin = io::stdin();
	let mut input_text: String = String::new();
	let result: Result<usize, io::Error> = reader.read_line(&mut input_text);
	if result.is_err() {
		println!("failed to read from stdin");
		return;
	}
	let mut x = 0;
	let trimmed: &str = input_text.trim();
	let option: Option<i32> = trimmed.parse::<i32>().ok();
	match option {
			Some(i) => {
				x = i;
				println!("your integer input: {}", i);
			},
			None => println!("this was not an integer: {}", trimmed),
		};

	let number = match x {
			1 => "one",
			2 => "two",
			3 => "three",
			4 => "four",
			5 => "five",
			6 => "six",
			7 => "seven",
			8 => "eight",
			9 => "nine",
			10 => "ten",
			_ => "not in range",
		};
	println!("");
	println!("BTW: {} is {}", x, number);
}