use pest::Parser as _;

fn main() {
	let input_str = aoc::get_input!();
	let mut input = InputParser::parse(Rule::file, &input_str)
		.expect("input failed to parse");

	let file = input.next().unwrap();
	assert!(matches!(file.as_rule(), Rule::file));

	let mut file = file.into_inner();

	let row = file.next().unwrap().as_str().parse::<usize>().unwrap();
	let col = file.next().unwrap().as_str().parse::<usize>().unwrap();
	let target_coords = (row, col);

	let mut row = 1usize;
	let mut current_code = 20151125usize;

	let code = 'outer: loop {
		let mut current_row = row;
		let mut current_col = 1usize;

		loop {
			let coords = (current_row, current_col);

			if coords == target_coords {
				break 'outer current_code
			}

			current_code = (current_code * 252533) % 33554393;

			if current_col == row && current_row == 1 {
				row += 1;
				break
			}

			current_row -= 1;
			current_col += 1;
		}
	};

	println!("part 1: code on ({}, {}): {}", target_coords.0, target_coords.1, code);
}

#[derive(pest_derive::Parser)]
#[grammar = "parsers/y2015-d25.pest"]
pub struct InputParser;
