use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

fn main() {
	let data = Arc::new(Mutex::new(0));

	let (sender, receiver) = mpsc::channel();

	for _ in 0..10 {
	let (data, sender) = (data.clone(), sender.clone());

	thread::spawn(move || {
	let mut data = data.lock().unwrap();
	*data += 1;

	let _ = sender.send(*data);
	});
}

for _ in 0..10 {
println!("{}",receiver.recv().unwrap());
}
}