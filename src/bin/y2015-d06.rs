use pest::Parser as _;

fn main() {
	let input_str = aoc::get_input!();
	let mut input = InputParser::parse(Rule::file, &input_str)
		.expect("input failed to parse");

	let file = input.next().unwrap();
	assert!(matches!(file.as_rule(), Rule::file));

	let mut the_grid = aoc::set!();
	for line in file.clone().into_inner() {
		if matches!(line.as_rule(), Rule::EOI) { break }

		line.into_instruction()
			.apply_to_grid(&mut the_grid);
	}
	println!("part 1: number of lights lit: {}", the_grid.len());

	let mut ancient_nordic_elvish_grid = aoc::map!();
	for line in file.into_inner() {
		if matches!(line.as_rule(), Rule::EOI) { break }

		line.into_instruction()
			.apply_to_ancient_nordic_elvish_grid(&mut ancient_nordic_elvish_grid);
	}
	println!("part 2: total brightness: {}", ancient_nordic_elvish_grid.iter().map(|c| *c.1).sum::<usize>());
}

#[derive(pest_derive::Parser)]
#[grammar = "parsers/y2015-d06.pest"]
pub struct InputParser;

type Coord = (usize, usize);
type Brightness = usize;

#[derive(Clone)]
enum Instruction {
	On,
	Off,
	Toggle
}

trait PairExts {
	fn into_instruction(self) -> (Instruction, Coord, Coord);
}

impl<'h> PairExts for pest::iterators::Pair<'h, Rule> {
	fn into_instruction(self) -> (Instruction, Coord, Coord) {
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
	fn apply_to_grid(self, grid: &mut aoc::Set<Coord>);
	fn apply_to_ancient_nordic_elvish_grid(self, grid: &mut aoc::Map<Coord, Brightness>);
}

impl ApplyInstruction for (Instruction, Coord, Coord) {
	fn apply_to_grid(self, grid: &mut aoc::Set<Coord>) {
		let (instruction, coord1, coord2) = self;

		let process = match instruction {
			Instruction::On => {
				fn turn_on(coord: Coord, grid: &mut aoc::Set<Coord>) {
					grid.insert(coord);
				}
				turn_on
			}
			Instruction::Off => {
				fn turn_off(coord: Coord, grid: &mut aoc::Set<Coord>) {
					grid.remove(&coord);
				}
				turn_off
			}
			Instruction::Toggle => {
				fn toggle(coord: Coord, grid: &mut aoc::Set<Coord>) {
					if !grid.remove(&coord) {
						grid.insert(coord);
					}
				}
				toggle
			}
		};

		iter_grid(coord1, coord2, |coord| process(coord, grid));
	}

	fn apply_to_ancient_nordic_elvish_grid(self, grid: &mut aoc::Map<Coord, Brightness>) {
		let (instruction, coord1, coord2) = self;

		fn get_or_insert_0_brightness<'h>(
			grid: &'h mut aoc::Map<Coord, Brightness>,
			coord: &'_ Coord
		) -> &'h mut usize {
			if !grid.contains_key(coord) {
				grid.insert(*coord, 0);
			}
			grid.get_mut(coord).unwrap()
		}

		let process = match instruction {
			Instruction::On => {
				fn increase_brightness_1(coord: Coord, grid: &mut aoc::Map<Coord, Brightness>) {
					let brightness = get_or_insert_0_brightness(grid, &coord);
					*brightness += 1;
				}
				increase_brightness_1
			}
			Instruction::Off => {
				fn saturating_decrease_brightness_1(coord: Coord, grid: &mut aoc::Map<Coord, Brightness>) {
					let brightness = get_or_insert_0_brightness(grid, &coord);
					*brightness = brightness.saturating_sub(1);
				}
				saturating_decrease_brightness_1
			}
			Instruction::Toggle => {
				fn increase_brightness_2(coord: Coord, grid: &mut aoc::Map<Coord, Brightness>) {
					let brightness = get_or_insert_0_brightness(grid, &coord);
					*brightness += 2;
				}
				increase_brightness_2
			}
		};

		iter_grid(coord1, coord2, |coord| process(coord, grid));
	}
}

#[inline]
fn iter_grid<F>((x1, y1): Coord, (x2, y2): Coord, mut f: F)
where
	F: FnMut(Coord)
{
	for x in if x1 < x2 { x1..=x2 } else { x2..=x1 } {
		for y in if y1 < y2 { y1..=y2 } else { y2..=y1 } {
			f((x, y));
		}
	}
}
