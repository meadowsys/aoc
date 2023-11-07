use itertools::Itertools as _;

fn main() {
	let input_str = aoc::get_input!();
	let mut input = input_str.trim()
		.lines()
		.map(|s| s.trim())
		.map(|s| s.parse::<usize>().unwrap())
		.collect::<Vec<_>>();
	input.sort_unstable();

	let smol = get_smol(&input, 3);
	println!("part 1: smollest, ideal quantum entanglement: {smol}");

	let smol = get_smol(&input, 4);
	println!("part 2: smollest, ideal quantum entanglement, now with trunk: {smol}");
}

fn get_smol(input: &[usize], k: usize) -> usize {
	let weight_per_side = input.iter().sum::<usize>() / k;

	for i in 1..input.len() {
		let smol = input.iter()
			.permutations(i)
			.filter(|p| p.iter().copied().sum::<usize>() == weight_per_side)
			.map(|p| p.iter().copied().product::<usize>())
			.min();
		if let Some(smol) = smol {
			return smol
		}
	}

	unreachable!("aoc input should be valid??");
}
