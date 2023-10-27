use pest::Parser as _;

fn main() {
	let input_str = aoc::get_input!();
	let mut input = InputParser::parse(Rule::file, &input_str)
		.expect("input failed to parse");

	let file = input.next().unwrap();
	assert!(matches!(file.as_rule(), Rule::file));

	let direction_manager = file.clone()
		.into_inner()
		.fold(DirectionManager::new_solo(), feed_directions);
	let houses_with_present = direction_manager.count_houses_with_present();

	println!("part 1: houses that got a present: {houses_with_present}");

	let direction_manager = file.into_inner()
		.fold(DirectionManager::new_with_robo(), feed_directions);
	let houses_with_present = direction_manager.count_houses_with_present();

	println!("part 2: houses that got a present: {houses_with_present}");
}

#[derive(pest_derive::Parser)]
#[grammar = "parsers/y2015-d03.pest"]
pub struct InputParser;

#[derive(Clone)]
enum Direction { Up, Down, Left, Right }

#[derive(Clone)]
enum DirectionManager {
	Solo {
		/// (x, y)
		current_coords: (isize, isize),
		visited_coords: aoc::Map<(isize, isize), VisitationState>
	},
	WithRoboSanta {
		santa_coords: (isize, isize),
		robo_coords: (isize, isize),
		visited_coords: aoc::Map<(isize, isize), VisitationState>,
		turn: Turn
	}
}

#[derive(Clone)]
enum VisitationState {
	VisitedOnce,
	VisitedMultiple
}

#[derive(Clone)]
enum Turn {
	Santa,
	Robo
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
	fn new_solo() -> Self {
		let current_coords = (0, 0);
		let mut visited_coords = aoc::new_map();

		// visited current location first
		visited_coords.insert(current_coords, VisitationState::VisitedOnce);

		Self::Solo { current_coords, visited_coords }
	}

	fn new_with_robo() -> Self {
		let santa_coords = (0, 0);
		let robo_coords = (0, 0);
		let mut visited_coords = aoc::new_map();
		let turn = Turn::Santa;

		// visited current location first
		visited_coords.insert(santa_coords, VisitationState::VisitedOnce);

		Self::WithRoboSanta { santa_coords, robo_coords, visited_coords, turn }
	}

	/// returns if the just suppled direction resulted in that house having more
	/// than one present for the first time, so subsequent visits will return false
	fn next_instruction(&mut self, direction: Direction) {
		match self {
			DirectionManager::Solo { current_coords, visited_coords } => {
				match direction {
					Direction::Up => {
						current_coords.1 += 1;
					}
					Direction::Down => {
						current_coords.1 -= 1;
					}
					Direction::Left => {
						current_coords.0 += 1;
					}
					Direction::Right => {
						current_coords.0 -= 1;
					}
				}

				let new_state = match visited_coords.get(current_coords) {
					None => { VisitationState::VisitedOnce }
					Some(VisitationState::VisitedOnce) => { VisitationState::VisitedMultiple }
					Some(VisitationState::VisitedMultiple) => { VisitationState::VisitedMultiple }
				};
				visited_coords.insert(*current_coords, new_state);
			}
			DirectionManager::WithRoboSanta { santa_coords, robo_coords, visited_coords, turn } => {
				let current_coords = match turn {
					Turn::Santa => {
						*turn = Turn::Robo;
						santa_coords
					}
					Turn::Robo => {
						*turn = Turn::Santa;
						robo_coords
					}
				};

				match direction {
					Direction::Up => {
						current_coords.1 += 1;
					}
					Direction::Down => {
						current_coords.1 -= 1;
					}
					Direction::Left => {
						current_coords.0 += 1;
					}
					Direction::Right => {
						current_coords.0 -= 1;
					}
				}

				let new_state = match visited_coords.get(current_coords) {
					None => { VisitationState::VisitedOnce }
					Some(VisitationState::VisitedOnce) => { VisitationState::VisitedMultiple }
					Some(VisitationState::VisitedMultiple) => { VisitationState::VisitedMultiple }
				};
				visited_coords.insert(*current_coords, new_state);
			}
		}
	}

	fn count_houses_with_present(&self) -> usize {
		match self {
			DirectionManager::Solo { visited_coords, .. }
			| DirectionManager::WithRoboSanta { visited_coords, .. }
			=> {
				visited_coords.values().count()
			}
		}
	}
}

fn feed_directions(mut direction_manager: DirectionManager, d: pest::iterators::Pair<Rule>) -> DirectionManager {
	if matches!(d.as_rule(), Rule::EOI) { return direction_manager }
	direction_manager.next_instruction(d.to_direction());
	direction_manager
}
