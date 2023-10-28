use pest::Parser as _;

fn main() {
	let input_str = aoc::get_input!();
	let mut input = InputParser::parse(Rule::file, &input_str)
		.expect("input failed to parse");

	let file = input.next().unwrap();
	assert!(matches!(file.as_rule(), Rule::file));

	let mut the_grid = aoc::set!();

	for line in file.into_inner() {
		if matches!(line.as_rule(), Rule::EOI) { break }

		line.into_instruction()
			.apply_to_grid(&mut the_grid);
	}

	println!("part 1: number of lights lit: {}", the_grid.len());
}

#[derive(pest_derive::Parser)]
#[grammar = "parsers/y2015-d06.pest"]
pub struct InputParser;

#[derive(Clone)]
enum Instruction {
	On,
	Off,
	Toggle
}

trait PairExts {
	fn into_instruction(self) -> (Instruction, (usize, usize), (usize, usize));
}

impl<'h> PairExts for pest::iterators::Pair<'h, Rule> {
	fn into_instruction(self) -> (Instruction, (usize, usize), (usize, usize)) {
		let instruction = match self.as_rule() {
			Rule::on => { Instruction::On }
			Rule::off => { Instruction::Off }
			Rule::toggle => { Instruction::Toggle }
			_ => { unreachable!() }
		};
		let mut inner = self.into_inner();

		let x = inner.next().unwrap().as_span().as_str().parse().unwrap();
		let y = inner.next().unwrap().as_span().as_str().parse().unwrap();

		let coord1 = (x, y);

		let x = inner.next().unwrap().as_span().as_str().parse().unwrap();
		let y = inner.next().unwrap().as_span().as_str().parse().unwrap();
		let coord2 = (x, y);

		(instruction, coord1, coord2)
	}
}

trait ApplyInstruction {
	fn apply_to_grid(self, grid: &mut aoc::Set<(usize, usize)>);
}

impl ApplyInstruction for (Instruction, (usize, usize), (usize, usize)) {
	fn apply_to_grid(self, grid: &mut aoc::Set<(usize, usize)>) {
		let (instruction, (x1, y1), (x2, y2)) = self;

		let process = match instruction {
			Instruction::On => {
				fn turn_on(x: usize, y: usize, grid: &mut aoc::Set<(usize, usize)>) {
					grid.insert((x, y));
				}
				turn_on
			}
			Instruction::Off => {
				fn turn_off(x: usize, y: usize, grid: &mut aoc::Set<(usize, usize)>) {
					grid.remove(&(x, y));
				}
				turn_off
			}
			Instruction::Toggle => {
				fn toggle(x: usize, y: usize, grid: &mut aoc::Set<(usize, usize)>) {
					if !grid.remove(&(x, y)) {
						grid.insert((x, y));
					}
				}
				toggle
			}
		};

		for x in if x1 < x2 { x1..=x2 } else { x2..=x1 } {
			for y in if y1 < y2 { y1..=y2 } else { y2..=y1 } {
				process(x, y, grid);
			}
		}
	}
}
