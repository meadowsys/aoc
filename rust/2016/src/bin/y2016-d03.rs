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

	let valid_triangles = lines.clone()
		.map(|l| l.into_triangle())
		.filter(|t| t.is_valid())
		.collect::<Vec<_>>();

	println!("part 1: number of valid triangles: {}", valid_triangles.len());

	let valid_triangles = lines.clone()
		.batching(|iter| match iter.next() {
			None => { None }
			Some(iter1) => {
				let iter1 = iter1.into_inner();
				let iter2 = iter.next().unwrap().into_inner();
				let iter3 = iter.next().unwrap().into_inner();

				let iter = iter1.zip(iter2).zip(iter3);

				let triangles = iter.take(3)
					.map(|((s1, s2), s3)| {
						let s1 = s1.as_str().parse::<usize>().unwrap();
						let s2 = s2.as_str().parse::<usize>().unwrap();
						let s3 = s3.as_str().parse::<usize>().unwrap();

						let mut sorter = [s1, s2, s3];
						sorter.sort_unstable();
						let [s1, s2, s3] = sorter;

						(s1, s2, s3)
					})
					.collect::<Vec<_>>();

				Some(triangles)
			}
		})
		.flatten()
		.map(|(s1, s2, s3)| Triangle(s1, s2, s3))
		.filter(|t| t.is_valid())
		.collect::<Vec<_>>();

	println!("part 2: number of valid triangles when col: {}", valid_triangles.len());
}

#[derive(pest_derive::Parser)]
#[grammar = "parsers/y2016-d03.pest"]
pub struct InputParser;

struct Triangle(usize, usize, usize);

impl Triangle {
	fn is_valid(&self) -> bool {
		let mut sorter = [self.0, self.1, self.2];
		sorter.sort_unstable();
		sorter[0] + sorter[1] > sorter[2]
	}
}

trait PairExts {
	fn into_triangle(self) -> Triangle;
}

impl<'h> PairExts for pest::iterators::Pair<'h, Rule> {
	fn into_triangle(self) -> Triangle {
		let mut inner = self.into_inner();

		let s1 = inner.next().unwrap().as_str().parse::<usize>().unwrap();
		let s2 = inner.next().unwrap().as_str().parse::<usize>().unwrap();
		let s3 = inner.next().unwrap().as_str().parse::<usize>().unwrap();

		Triangle(s1, s2, s3)
	}
}
