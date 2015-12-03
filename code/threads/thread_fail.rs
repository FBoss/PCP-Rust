//Source: http://doc.rust-lang.org/book/concurrency.html

use std::thread;

fn main() {
	let mut data = vec![1, 2, 3];

	for i in 0..3 {
	thread::spawn(move || {
	data[i] += 1;
	});
}

thread::sleep_ms(50);
}