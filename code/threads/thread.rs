use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
	let data = Arc::new(Mutex::new(vec![1, 2, 3]));

	//Does not work because concurrency violation
	//print!("Initial Values: data[0]={}, data[1]={}, data[2]={}, ", data[0],data[1],data[2]);

	print!("Initial Values: ");
	for i in 0..3 {
	let data = data.clone();
	let data = data.lock().unwrap();
	print!("data[{}]={}, ", i, data[i]);
}

println!("");

for i in 0..3 {
let data = data.clone();
thread::spawn(move || {
let mut data = data.lock().unwrap();
data[i] += 1;
println!("New value of data[{}] is {}", i, data[i]);
});
}

thread::sleep_ms(50);
}