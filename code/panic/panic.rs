fn main() {
    use std::thread;
    
    let handle = thread::spawn(move || {
        panic!("oops!");
    });
    
    let result = handle.join();
    println!("Thread Error: {}", result.is_err());
    
}