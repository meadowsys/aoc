use pest::Parser as _;
use std::cmp::Reverse;

fn main() {
	let input_str = aoc::get_input!();
	let mut input = InputParser::parse(Rule::file, &input_str)
		.expect("input failed to parse");

	let file = input.next().unwrap();
	assert!(matches!(file.as_rule(), Rule::file));

	let lines = file.into_inner()
		.take_while(|l| !matches!(l.as_rule(), Rule::EOI));

	let real_rooms = lines
		.filter_map(into_valid_room_sector_id)
		.collect::<Vec<_>>();

	let real_room_count = real_rooms.iter()
		.map(|(i, _)| i)
		.sum::<usize>();

	println!("part 1: real rooms: {real_room_count}");

	let rotated_strings = real_rooms.iter()
		.map(|(sector, name)| (
			*sector,
			name.chars().map(|c| rotate(c, *sector)).collect::<String>()
		))
		.collect::<Vec<_>>();

	println!("part 2: rotated strings:");
	for (sector, name) in &rotated_strings {
		println!("   sector {sector}, {name}");
	}
	println!("end part 2: rotated strings");
	println!("(try piping to `grep north` or something)");
}

#[derive(pest_derive::Parser)]
#[grammar = "parsers/y2016-d04.pest"]
pub struct InputParser;

fn into_valid_room_sector_id(
	line: pest::iterators::Pair<'_, Rule>
) -> Option<(usize, String)> {
	let mut string = String::with_capacity(line.as_str().len());
	let mut inner = line.into_inner();

	loop {
		let item = inner.next().unwrap();

		if !matches!(item.as_rule(), Rule::letter_group) {
			string.pop();

			let letter_group_5 = inner.next()
				.unwrap()
				.as_str();

			break if is_valid(&string, letter_group_5) {
				let sector_id = item.as_str()
					.parse::<usize>()
					.unwrap();
				Some((sector_id, string))
			} else {
				None
			}
		}

		string.push_str(item.as_str());
		string.push(' ');
	}
}

fn is_valid(letters: &str, most: &str) -> bool {
	assert_eq!(most.len(), 5);
	assert_eq!(most.chars().collect::<aoc::Set<_>>().len(), 5);

	let mut map = aoc::map!();
	for char in letters.chars() {
		let entry = map.entry(char)
			.or_insert(0usize);
		*entry += 1;
	}

	let mut vec = map.into_iter()
		.collect::<Vec<_>>();
	vec.sort_unstable_by_key(|(letter, count)| (Reverse(*count), *letter));

	vec.into_iter()
		.filter(|(c, _)| c.is_alphabetic())
		.take(5)
		.all(|c| most.contains(c.0))
}

/// n must be less than 26 (num chars in alpha)
fn rotate(c: char, n: usize) -> char {
	match c {
		'a'..='z' => {
			let mut c = c as usize - b'a' as usize;
			c += n;
			c %= 26;
			(c as u8 + b'a') as char
		}
		' ' => { ' ' }
		_ => { unreachable!() }
	}
}
