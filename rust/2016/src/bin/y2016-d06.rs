use itertools::Itertools as _;
use std::cmp::Reverse;

fn main() {
	let input = aoc::get_input!();
	let input = input.trim()
		.lines()
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let mut cols = vec![vec![]; input.first().unwrap().len()];

	input.iter()
		.for_each(|line| {
			line.chars()
				.zip(cols.iter_mut())
				.for_each(|(char, col_vec)| {
					col_vec.push(char);
				})
		});

	let all_equal_len = cols.iter()
		.map(|col| col.len())
		.all_equal();
	assert!(all_equal_len);

	let message = cols.iter()
		.map(|col| {
			let mut char_counts = col.iter()
				.fold(aoc::map!(), |mut map, next| {
					*map.entry(*next).or_insert(0usize) += 1;
					map
				})
				.into_iter()
				.collect::<Vec<_>>();
			char_counts.sort_unstable_by_key(|(char, count)| (Reverse(*count), *char));
			char_counts.first().unwrap().0
		})
		.collect::<String>();

	println!("part 1: decoded message: {message}");
}

// fn get_max_occurance
