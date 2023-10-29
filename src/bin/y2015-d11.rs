fn main() {
	let input_str = aoc::get_input!();
	let mut input = input_str.trim().chars().rev().collect::<String>();

	let testdata = build_testdata();

	while !test(&input, &testdata) {
		increment_reversed_input(&mut input);
	}

	println!("part 1: next password: \"{}\"", input.chars().rev().collect::<String>());
}

const I: u8 = b'i';
const O: u8 = b'o';
const L: u8 = b'l';
const FORBIDDEN_LETTERS: [u8; 3] = [I, O, L];

fn increment_reversed_input(str: &mut String) {
	let mut increment = true;

	let new_str = str.chars()
		.map(|c| if !increment {
			c
		} else {
			increment = false;
			match c {
				c @ 'a'..='y' => {
					let c = c as u8 + 1;
					let c = if FORBIDDEN_LETTERS.contains(&c) {
						c + 1
					} else {
						c
					};
					c as char
				}
				'z' => {
					increment = true;
					'a'
				}
				_ => { unreachable!() }
			}
		})
		.collect();
	*str = new_str;
}

struct TestData {
	seq_3: Vec<String>,
	pairs: Vec<String>
}

fn build_testdata() -> TestData {
	let chars = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>();
	let seq_3 = chars.windows(3)
		.map(|c| c.iter().rev().collect::<String>())
		.filter(|s| FORBIDDEN_LETTERS.iter().all(|c| !s.contains(*c as char)))
		.collect();

	let pairs = chars.iter()
		.map(|c| c.to_string().repeat(2))
		.filter(|s| FORBIDDEN_LETTERS.iter().all(|c| !s.contains(*c as char)))
		.collect();

	TestData { seq_3, pairs }
}

fn test(str: &str, data: &TestData) -> bool {
	let TestData { seq_3, pairs } = data;

	let seq = seq_3.iter().any(|seq| str.contains(seq));
	let pairs = pairs.iter().filter(|pair| str.contains(*pair)).count() >= 2;

	seq && pairs
}
