use itertools::Itertools as _;

fn main() {
	let input_str = aoc::get_input!();
	let mut input = input_str.trim()
		.lines()
		.map(|s| s.trim())
		.map(|s| s.parse::<usize>().unwrap())
		.collect::<Vec<_>>();
	input.sort_unstable();
	let weight_per_side = input.iter().sum::<usize>() / 3;

	for i in 1..input.len() {
		let small = input.iter()
			.permutations(i)
			.filter(|p| p.iter().copied().sum::<usize>() == weight_per_side)
			.map(|p| p.iter().copied().product::<usize>())
			.min();
		if let Some(small) = small {
			println!("part 1: smallest quantum entanglement in ideal arrangement: {small}");
			break
		}
	}
}
