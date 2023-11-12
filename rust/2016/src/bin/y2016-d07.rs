#![allow(clippy::ptr_arg)]

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

	let tls = segmented_lines.iter()
		.filter(|segments| {
			let (outside, inside) = segments.iter()
				.partition_map::<Vec<_>, Vec<_>, _, _, _>(outside_left);

			let out_result = || outside.into_iter().any(|s| has_abba(s));
			let in_result = || inside.into_iter().all(|s| !has_abba(s));

			out_result() && in_result()
		})
		.collect::<Vec<_>>();

	println!("part 1: IPs that support TLS: {}", tls.len());

	let ssl = segmented_lines.iter()
		.filter(|segments| {
			let (outside, inside) = segments.iter()
				.partition_map::<Vec<_>, Vec<_>, _, _, _>(outside_left);

			let out_bab = outside.iter()
				.copied()
				.flat_map(get_aba)
				.map(aba_to_bab)
				.map(bab_to_str)
				.collect::<Vec<_>>();

			if out_bab.is_empty() {
				false
			} else {
				inside.iter()
					.any(|s| out_bab.iter().any(|bab| s.contains(bab)))
			}
		})
		.collect::<Vec<_>>();

		println!("part 2: IPs that support SSL: {}", ssl.len());
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

fn get_aba(s: &String) -> Vec<(char, char, char)> {
	s.chars()
		.tuple_windows::<(_, _, _)>()
		.filter(|(a, b, c)| a != b && a == c)
		.collect()
}

fn aba_to_bab(aba: (char, char, char)) -> (char, char, char) {
	let (a, b, _a) = aba;
	(b, a, b)
}

fn bab_to_str(bab: (char, char, char)) -> String {
	[bab.0, bab.1, bab.2].into_iter().collect()
}

fn outside_left(s: &Segment) -> Either<&String, &String> {
	match s {
		Segment::OutsideBracket(s) => { Either::Left(s) }
		Segment::InsideBracket(s) => { Either::Right(s) }
	}
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
