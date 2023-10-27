use pest::Parser as _;

fn main() {
	let input_str = aoc::get_input!();
	let mut input = InputParser::parse(Rule::file, &input_str)
		.expect("input failed to parse");

	let file = input.next().unwrap();
	assert!(matches!(file.as_rule(), Rule::file));

	let inner_file = file.into_inner();
	let before_eoi = inner_file.take_while(|d| !matches!(d.as_rule(), Rule::EOI));

	let floor = before_eoi.clone()
		.fold(0isize, |a, c| a + c.get_increment());
	println!("part 1: floor {floor}");

	let mut current_floor = 0isize;
	for (direction, i) in before_eoi.zip(1..) {
		current_floor += direction.get_increment();
		if current_floor == -1 {
			println!("part 2: step {i}");
			break
		}
	}
}

#[derive(pest_derive::Parser)]
#[grammar = "parsers/y2015-d01.pest"]
pub struct InputParser;

trait PairExts {
	fn get_increment(&self) -> isize;
}

impl<'h> PairExts for pest::iterators::Pair<'h, Rule> {
	fn get_increment(&self) -> isize {
		match self.as_span().as_str() {
			"(" => { 1 }
			")" => { -1 }
			_ => unreachable!()
		}
	}
}
