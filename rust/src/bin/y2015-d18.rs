fn main() {
	let input_str = aoc::get_input!();
	let coords = input_str.trim()
		.split('\n')
		.map(|l| l.trim())
		.zip(0isize..)
		.flat_map(|(l, y)| {
			l.bytes()
				.zip(0isize..)
				.filter_map(move |(b, x)| match b {
					b'#' => { Some((x, y)) }
					b'.' => { None }
					_ => { unreachable!() }
				})
		})
		.collect::<aoc::Set<_>>();

	let bounds = (100isize, 100isize);

	let num_iterations = 100usize;

	let mut new_coords = coords.clone();
	for _ in 0..num_iterations {
		new_coords = step(&new_coords, bounds, false);
	}
	println!("part 1: number of lights on: {}", new_coords.len());

	let mut new_coords = coords.clone();
	force_corners_on(&mut new_coords, bounds);
	for _ in 0..num_iterations {
		new_coords = step(&new_coords, bounds, true);
	}
	println!("part 2: number of lights on (corners forced on): {}", new_coords.len());
}

fn step(map: &aoc::Set<(isize, isize)>, (x_bound, y_bound): (isize, isize), should_force_corners_on: bool) -> aoc::Set<(isize, isize)> {
	let mut next = aoc::set!();
	let is_considered_on = |x, y| {
		if x < 0 || y < 0 || x >= x_bound || y >= y_bound {
			false
		} else {
			map.contains(&(x, y))
		}
	};

	for x in 0..x_bound {
		for y in 0..y_bound {
			let currently_on = is_considered_on(x, y);

			let surrounding = [
				is_considered_on(x + 1, y),
				is_considered_on(x - 1, y),
				is_considered_on(x, y + 1),
				is_considered_on(x, y - 1),
				is_considered_on(x + 1, y + 1),
				is_considered_on(x + 1, y - 1),
				is_considered_on(x - 1, y + 1),
				is_considered_on(x - 1, y - 1),
			];

			let surrounding_on_count = surrounding.into_iter().filter(|x| *x).count();
			let next_on = if currently_on {
				surrounding_on_count == 2 || surrounding_on_count == 3
			} else {
				surrounding_on_count == 3
			};

			if next_on {
				next.insert((x, y));
			}
		}
	}

	if should_force_corners_on { force_corners_on(&mut next, (x_bound, y_bound)) }
	next
}

fn force_corners_on(map: &mut aoc::Set<(isize, isize)>, (x_bound, y_bound): (isize, isize)) {
	map.insert((0, 0));
	map.insert((x_bound - 1, 0));
	map.insert((0, y_bound - 1));
	map.insert((x_bound - 1, y_bound - 1));
}
