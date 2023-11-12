use itertools::Either;
use itertools::Itertools as _;
use pest::Parser as _;

fn main() {
	let input_str = aoc::get_input!();
	let mut input = InputParser::parse(Rule::file, &input_str)
		.expect("input failed to parse");

	let file = input.next().unwrap();
	assert!(matches!(file.as_rule(), Rule::file));

	let lines = file.into_inner()
		.take_while(|l| !matches!(l.as_rule(), Rule::EOI));

	let segmented_lines = lines
		.map(PairExts::into_segments)
		.collect::<Vec<_>>();

	let lines = segmented_lines.iter()
		.filter(|segments| {
			let (outside, inside) = segments.iter()
				.partition_map::<Vec<_>, Vec<_>, _, _, _>(|s| match s {
					Segment::OutsideBracket(s) => { Either::Left(s) }
					Segment::InsideBracket(s) => { Either::Right(s) }
				});

			let out_result = || outside.into_iter().any(|s| has_abba(s));
			let in_result = || inside.into_iter().all(|s| !has_abba(s));

			out_result() && in_result()
		})
		.collect::<Vec<_>>();

	println!("part 1: IPs that support TLS: {}", lines.len());
}

enum Segment {
	OutsideBracket(String),
	InsideBracket(String)
}

#[derive(pest_derive::Parser)]
#[grammar = "parsers/y2016-d07.pest"]
pub struct InputParser;

fn has_abba(s: &str) -> bool {
	s.chars()
		.tuple_windows::<(_, _, _, _)>()
		.any(|(a, b, c, d)| a != b && a == d && b == c)
}

trait PairExts {
	fn into_segments(self) -> Vec<Segment>;
	fn into_segment(self) -> Segment;
}

impl<'h> PairExts for pest::iterators::Pair<'h, Rule> {
	fn into_segments(self) -> Vec<Segment> {
		self.into_inner()
			.map(|pair| pair.into_segment())
			.collect()
	}

	fn into_segment(self) -> Segment {
		let segment = self.as_str().into();
		match self.as_rule() {
			Rule::outside_bracket => { Segment::OutsideBracket(segment) }
			Rule::inside_bracket => { Segment::InsideBracket(segment) }
			_ => { unreachable!() }
		}
	}
}
