use std::fmt;

fn main() {
	let input_str = aoc::get_input!();

	let mut state = State::new();

	input_str
		.trim()
		.split(", ")
		.map(|s| {
			let s = s.trim();
			let direction = &s[..1];
			let amount = &s[1..];

			let direction = match direction {
				"L" => { Direction::Left }
				"R" => { Direction::Right }
				_ => { unreachable!() }
			};
			let amount = amount.parse::<isize>().unwrap();

			(direction, amount)
		})
		.for_each(|input| state.process(input));

	let total_distance = state.distance_x + state.distance_y;
	println!("part 1: total distance: {total_distance}");
}

enum Direction {
	Up,
	Right,
	Down,
	Left
}

/// imagine coordinate grid
struct State {
	direction: Direction,
	distance_x: isize,
	distance_y: isize
}

impl State {
	fn new() -> Self {
		Self {
			direction: Direction::Up,
			distance_x: 0,
			distance_y: 0
		}
	}

	fn process(&mut self, input: (Direction, isize)) {
		let (direction, amount) = input;

		use Direction::*;
		self.direction = match direction {
			Left => match self.direction {
				Up => { Left }
				Right => { Up }
				Down => { Right }
				Left => { Down}
			}
			Right => match self.direction {
				Up => { Right }
				Right => { Down }
				Down => { Left }
				Left => { Up }
			}
			Up | Down => { unreachable!() }
		};

		match self.direction {
			Up => { self.distance_y += amount }
			Right => { self.distance_x += amount }
			Down => { self.distance_y -= amount }
			Left => { self.distance_x -= amount }
		}
	}
}
