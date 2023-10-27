use md5::{ Md5, Digest };

fn main() {
	let input_str = aoc::get_input!();
	let key = input_str.trim();

	for i in 0usize.. {
		let hash_input = format!("{key}{i}");

		let mut hasher = Md5::new();
		hasher.update(hash_input);
		let result = Into::<[u8; 16]>::into(hasher.finalize())
			.map(|e| format!("{e:02x?}"))
			.join("");
		if result.starts_with("00000") {
			println!("part 1: first hash with 5 starting zeros: {i} ({result})");
			break
		}
	}
}
