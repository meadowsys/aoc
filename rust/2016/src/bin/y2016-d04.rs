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
		.sum::<usize>();

	println!("part 1: real rooms: {real_rooms}");
}

#[derive(pest_derive::Parser)]
#[grammar = "parsers/y2016-d04.pest"]
pub struct InputParser;

fn into_valid_room_sector_id(
	line: pest::iterators::Pair<'_, Rule>
) -> Option<usize> {
	let mut string = String::with_capacity(line.as_str().len());
	let mut inner = line.into_inner();

	loop {
		let item = inner.next().unwrap();

		if !matches!(item.as_rule(), Rule::letter_group) {
			let letter_group_5 = inner.next()
				.unwrap()
				.as_str();

			break if is_valid(&string, letter_group_5) {
				let sector_id = item.as_str()
					.parse::<usize>()
					.unwrap();
				Some(sector_id)
			} else {
				None
			}
		}

		string.push_str(item.as_str());
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
		.take(5)
		.all(|c| most.contains(c.0))
}
