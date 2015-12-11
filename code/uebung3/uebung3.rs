fn sort_vec_int(mut f: Vec<u32>){
	println!("---------------------------------------------------");
	f.sort();
	for x in f.iter() {
		println!("{}", x );
		};
	println!("---------------------------------------------------");
}

fn sort_vec_str(mut f: Vec<&str>){
	println!("---------------------------------------------------");
		f.sort();
	for x in f.iter() {
		println!("{}", x );
	};
	println!("---------------------------------------------------");
}



fn main() {
	let vec1 = vec![4, 3, 1, 3892, 12];
	let vec2 = vec!["aaaa", "bbb","ccds","ccas", "aadsfa", "bcka", "1gds3"];
	sort_vec_int(vec1);
	sort_vec_str(vec2);
}