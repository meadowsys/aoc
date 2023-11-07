fn main() {
	let input_str = aoc::get_input!();

	let directions = input_str
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
		.collect::<Vec<_>>();

	let mut state = State::new();
	directions.iter()
		.for_each(|input| { state.process(input, false); });

	let total_distance = state.distance_x.abs() + state.distance_y.abs();
	println!("part 1: total distance: {total_distance}");

	let mut state = State::new();
	let mut found = false;

	for input in directions.iter() {
		found = state.process(input, true);
		if found { break }
	}

	if !found { panic!("notfoundaa"); }
	let total_distance = state.distance_x.abs() + state.distance_y.abs();
	println!("part 2: total distance of first dupe: {total_distance}");
}

#[derive(Debug)]
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
	distance_y: isize,
	coords_cache: aoc::Set<(isize, isize)>
}

impl State {
	fn new() -> Self {
		Self {
			direction: Direction::Up,
			distance_x: 0,
			distance_y: 0,
			coords_cache: aoc::set!()
		}
	}

	fn cache_self(&mut self) -> bool {
		!self.coords_cache.insert((self.distance_x, self.distance_y))
	}

	/// returns true when the resulting coords that just got processed
	/// puts you onto a spot you've been on before
	fn process(&mut self, input: &(Direction, isize), stop_early: bool) -> bool {
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

		let (x_increment, y_increment) = match self.direction {
			Up => { (0, 1) }
			Right => { (1, 0) }
			Down => { (0, -1) }
			Left => { (-1, 0) }
		};

		for _ in 0..*amount {
			self.distance_x += x_increment;
			self.distance_y += y_increment;
			let res = self.cache_self();
			if res && stop_early { return true }
		}

		false
	}
}
