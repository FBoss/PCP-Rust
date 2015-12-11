fn search_dividable3(f: Vec<u32>){
	println!("---------------------------------------------------");
	for x in f.iter() {
		if x%3==0 {
			println!("{}", x );
		};
	}
	println!("---------------------------------------------------");
}

fn main() {
	let vec1 = vec![1, 2, 3, 4, 6, 7, 8, 9, 281, 1250, 7944];
	search_dividable3(vec1);
}