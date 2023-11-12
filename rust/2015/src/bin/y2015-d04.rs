use md5::{ Md5, Digest };

fn main() {
	let input_str = aoc::get_input!();
	let key = input_str.trim();

	let (i, result) = get_first_hash_starting_with(key, "00000");
	println!("part 1: first hash with 5 starting zeros: {i} ({result})");

	let (i, result) = get_first_hash_starting_with(key, "000000");
	println!("part 2: first hash with 6 starting zeros: {i} ({result})");
}

fn get_first_hash_starting_with(key: &str, starting_with: &str) -> (usize, String) {
	for i in 0usize.. {
		let hash_input = format!("{key}{i}");

		let mut hasher = Md5::new();
		hasher.update(hash_input);

		let result = Into::<[u8; 16]>::into(hasher.finalize())
			.map(|e| format!("{e:02x?}"))
			.join("");

		if result.starts_with(starting_with) {
			return (i, result)
		}
	}

	unreachable!()
}
