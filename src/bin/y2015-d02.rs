use pest::Parser as _;

fn main() {
	let input_str = aoc::get_input!();
	let mut input = InputParser::parse(Rule::file, &input_str)
		.expect("input failed to parse");

	let file = input.next().unwrap();
	assert!(matches!(file.as_rule(), Rule::file));

	let total_area = file.into_inner().fold(0usize, |acc, b| {
		if matches!(b.as_rule(), Rule::EOI) { return acc }

		let (l, w, h) = b.into_lwh();

		let s1 = l * w;
		let s2 = w * h;
		let s3 = h * l;

		let min_side = [s1, s2, s3].iter()
			.copied()
			.fold(usize::MAX, |a, c| a.min(c));

		acc + ([s1, s2, s3].into_iter().sum::<usize>() * 2) + min_side
	});

	println!("part 1: total area: {total_area}");
}

#[derive(pest_derive::Parser)]
#[grammar = "parsers/y2015-d02.pest"]
pub struct InputParser;

trait PairExts {
	fn into_lwh(self) -> (usize, usize, usize);
}

impl<'h> PairExts for pest::iterators::Pair<'h, Rule> {
	fn into_lwh(self) -> (usize, usize, usize) {
		let mut iter = self.into_inner()
			.map(|d| d.as_span().as_str().parse::<usize>().unwrap());

		let l = iter.next().unwrap();
		let w = iter.next().unwrap();
		let h = iter.next().unwrap();
		assert!(iter.next().is_none());
		(l, w, h)
	}
}
