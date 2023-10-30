use pest::Parser as _;

fn main() {
	let input_str = aoc::get_input!();
	let mut input = InputParser::parse(Rule::file, &input_str)
		.expect("input failed to parse");

	let file = input.next().unwrap();
	assert!(matches!(file.as_rule(), Rule::file));

	let mut reindeer = file.into_inner()
		.take_while(|l| !matches!(l.as_rule(), Rule::EOI))
		.map(|l| l.into_reindeer())
		.collect::<Vec<_>>();

	let time = 2503usize;
	for _ in 0..time {
		reindeer.iter_mut()
			.for_each(|r| r.step());
	}

	let max_distance = reindeer.iter()
		.map(|r| r.distance_travelled)
		.max()
		.unwrap();

	println!("part 1: max distance travelled: {max_distance}");
}

#[derive(pest_derive::Parser)]
#[grammar = "parsers/y2015-d14.pest"]
pub struct InputParser;

#[derive(Clone)]
struct Reindeer {
	name: String,
	speed: usize,
	active_time: usize,
	idle_time: usize,
	iter_state: IterState,
	distance_travelled: usize
}

#[derive(Clone)]
enum IterState {
	Active {
		elapsed_seconds: usize
	},
	Idle {
		elapsed_seconds: usize
	}
}

impl Default for IterState {
	fn default() -> Self {
		Self::Active { elapsed_seconds: 0 }
	}
}

impl Reindeer {
	fn step(&mut self) {
		let new_iter_state = match self.iter_state {
			IterState::Active { elapsed_seconds } => {
				self.distance_travelled += self.speed;

				let elapsed_seconds = elapsed_seconds + 1;
				if elapsed_seconds == self.active_time {
					IterState::Idle { elapsed_seconds: 0 }
				} else {
					IterState::Active { elapsed_seconds }
				}
			}
			IterState::Idle { elapsed_seconds } => {
				let elapsed_seconds = elapsed_seconds + 1;
				if elapsed_seconds == self.idle_time {
					IterState::Active { elapsed_seconds: 0 }
				} else {
					IterState::Idle { elapsed_seconds }
				}
			}
		};
		self.iter_state = new_iter_state;
	}
}

trait PairExts {
	fn into_reindeer(self) -> Reindeer;

	fn into_name(self) -> String;
	fn into_speed(self) -> usize;
	fn into_active_time(self) -> usize;
	fn into_idle_time(self) -> usize;
}

impl<'h> PairExts for pest::iterators::Pair<'h, Rule> {
	fn into_reindeer(self) -> Reindeer {
		let mut inner = self.into_inner();

		let name = inner.next().unwrap().into_name();
		let speed = inner.next().unwrap().into_speed();
		let active_time = inner.next().unwrap().into_active_time();
		let idle_time = inner.next().unwrap().into_idle_time();
		assert!(inner.next().is_none());

		Reindeer { name, speed, active_time, idle_time, iter_state: IterState::default(), distance_travelled: 0 }
	}

	#[inline]
	fn into_name(self) -> String {
		assert!(matches!(self.as_rule(), Rule::name));
		self.as_span().as_str().into()
	}

	#[inline]
	fn into_speed(self) -> usize {
		assert!(matches!(self.as_rule(), Rule::speed));
		self.as_span().as_str().parse().unwrap()
	}

	#[inline]
	fn into_active_time(self) -> usize {
		assert!(matches!(self.as_rule(), Rule::active_time));
		self.as_span().as_str().parse().unwrap()
	}

	#[inline]
	fn into_idle_time(self) -> usize {
		assert!(matches!(self.as_rule(), Rule::idle_time));
		self.as_span().as_str().parse().unwrap()
	}
}
