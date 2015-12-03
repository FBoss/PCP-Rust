use std::io;

fn guess(n: i32) -> bool {
	if n < 1 || n > 10 {
		panic!("Invalid number: {}", n);
	}
	n == 5
}

fn main() {
	println!("Input a Number between 1 and 10");

	let reader: io::Stdin = io::stdin();
	let mut input_text: String = String::new();
	let result: Result<usize, io::Error> = reader.read_line(&mut input_text);
	if result.is_err()
	{
		panic!("Failed to read from stdin");
	}

	let trimmed: &str = input_text.trim();
	let option: Option<i32> = trimmed.parse::<i32>().ok();

	match option {
			Some(i) => guess(i),
			None => panic!("{} is not a number!", trimmed)
		};

	println!("Your number is: {}", 3);
}