fn main() {
	let input = aoc::get_input!();
	let input = input.trim();

	let password = HashState::with_input(input)
		.take(8)
		.collect::<String>();

	println!("part 1: 8char password: {password}")
}


struct HashState {
	next_count: usize,
	input: String
}

impl HashState {
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

impl Iterator for HashState {
	type Item = char;
	fn next(&mut self) -> Option<Self::Item> {
		'outer: loop {
			let count_str = self.next_count.to_string();
			self.next_count += 1;

			self.input.push_str(&count_str);

			let mut hash = aoc::hash_md5(&self.input)
				.into_bytes()
				.into_iter();

			for _ in 0..count_str.len() {
				self.input.pop();
			}

			for _ in 0..5 {
				if hash.next().unwrap() != b'0' {
					continue 'outer
				}
			}

			return Some(hash.next().unwrap() as char)
		}
	}
}
