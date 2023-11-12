fn main() {
	let input_str = aoc::get_input!();
	let input = input_str.trim();

	let password = InterestingHashesIter::with_input(input)
		.take(PASSWORD_LEN)
		.map(|h| h.chars().nth(BEGINNING_ZEROS.len()).unwrap())
		.collect::<String>();

	println!("part 1: {PASSWORD_LEN}char password: {password}");

	let mut password_chars = aoc::map!();
	let range = 0..PASSWORD_LEN;

	let mut iter = InterestingHashesIter::with_input(input);
	while password_chars.len() < PASSWORD_LEN {
		let next_entry = iter.next()
			.unwrap();
		let mut next_entry = next_entry
			.chars()
			.skip(BEGINNING_ZEROS.len());

		// 0 to f
		let k = next_entry.next().unwrap() as u8;
		let v = next_entry.next().unwrap();

		let k = (k - b'0') as usize;
		if !range.contains(&k) { continue }

		password_chars.entry(k)
			.or_insert(v);
	}

	let password = range.into_iter()
		.map(|i| password_chars[&i])
		.collect::<String>();

	println!("part 2: {PASSWORD_LEN}char positional password: {password}");
}

const PASSWORD_LEN: usize = 8;
const BEGINNING_ZEROS: &str = "00000";

struct InterestingHashesIter {
	next_count: usize,
	input: String
}

impl InterestingHashesIter {
	fn with_input(input: &str) -> Self {
		Self {
			next_count: 0,
			input: {
				let mut str = String::with_capacity(input.len() + 1);
				str.push_str(input);
				str
			}
		}
	}
}

impl Iterator for InterestingHashesIter {
	type Item = String;
	fn next(&mut self) -> Option<Self::Item> {
		loop {
			let count_str = self.next_count.to_string();
			self.next_count += 1;

			self.input.push_str(&count_str);

			let hash = aoc::hash_md5(&self.input);

			for _ in 0..count_str.len() {
				self.input.pop();
			}

			if &hash[0..BEGINNING_ZEROS.len()] == BEGINNING_ZEROS {
				return Some(hash)
			}
		}
	}
}
