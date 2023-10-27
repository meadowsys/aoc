use pest::Parser as _;

fn main() {
	let input_str = aoc::get_input!();
	let mut input = InputParser::parse(Rule::file, &input_str)
		.expect("input failed to parse");

	let file = input.next().unwrap();
	assert!(matches!(file.as_rule(), Rule::file));

	let direction_manager = file.into_inner().fold(
		DirectionManager::new(),
		|mut direction_manager, d| {
			if matches!(d.as_rule(), Rule::EOI) { return direction_manager }
			direction_manager.next_instruction(d.to_direction());
			direction_manager
		}
	);
	let duplicate_houses = direction_manager.visited_coords.values()
		.filter(|v| matches!(v, VisitationState::VisitedMultiple))
		.count();

	println!("part 1: houses with 2+ presents: {duplicate_houses}");
}

#[derive(pest_derive::Parser)]
#[grammar = "parsers/y2015-d03.pest"]
pub struct InputParser;

#[derive(Clone)]
enum Direction { Up, Down, Left, Right }

#[derive(Clone)]
struct DirectionManager {
	/// (x, y)
	current_coords: (isize, isize),
	visited_coords: aoc::Map<(isize, isize), VisitationState>
}

#[derive(Clone)]
enum VisitationState {
	VisitedOnce,
	VisitedMultiple
}

trait PairExts {
	fn to_direction(&self) -> Direction;
}

impl<'h> PairExts for pest::iterators::Pair<'h, Rule> {
	fn to_direction(&self) -> Direction {
		match self.as_span().as_str() {
			"^" => { Direction::Up }
			"v" => { Direction::Down }
			"<" => { Direction::Left }
			">" => { Direction::Right }
			_ => unreachable!()
		}
	}
}

impl DirectionManager {
	fn new() -> Self {
		let current_coords = (0, 0);
		let mut visited_coords = aoc::new_map();

		// visited current location first
		visited_coords.insert(current_coords, VisitationState::VisitedOnce);

		Self { current_coords, visited_coords }
	}
	/// returns if the just suppled direction resulted in that house having more
	/// than one present for the first time, so subsequent visits will return false
	fn next_instruction(&mut self, direction: Direction) {
		match direction {
			Direction::Up => {
				self.current_coords.1 += 1;
			}
			Direction::Down => {
				self.current_coords.1 -= 1;
			}
			Direction::Left => {
				self.current_coords.0 += 1;
			}
			Direction::Right => {
				self.current_coords.0 -= 1;
			}
		}

		let new_state = match self.visited_coords.get(&self.current_coords) {
			None => { VisitationState::VisitedOnce }
			Some(VisitationState::VisitedOnce) => { VisitationState::VisitedMultiple }
			Some(VisitationState::VisitedMultiple) => { VisitationState::VisitedMultiple }
		};

		self.visited_coords.insert(self.current_coords, new_state);
	}
}
