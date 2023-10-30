use pest::Parser as _;

fn main() {
	let input_str = aoc::get_input!();
	let mut input = InputParser::parse(Rule::file, &input_str)
		.expect("input failed to parse");

	let file = input.next().unwrap();
	assert!(matches!(file.as_rule(), Rule::file));

	let mut lines = file.into_inner()
		.take_while(|l| !matches!(l.as_rule(), Rule::EOI));

	let mut replacers = aoc::set!();

	let mut current = lines.next().unwrap();
	loop {
		if !matches!(current.as_rule(), Rule::replacement_def_line) { break }

		replacers.insert(current.into_replacer());
		current = lines.next().unwrap();
	}

	let molecule = current.as_str().to_string();

	let replaced_molecules = replacers.iter()
		.flat_map(|(pat, replacer)| {
			let mut cur = 0;
			let mut replaced_molecules = Vec::<String>::new();

			while let Some(i) = molecule[cur..].find(pat) {
				let start_point = cur + i;
				let cont_point = cur + i + pat.len();

				let string = molecule[..start_point].chars()
					.chain(replacer.chars())
					.chain(molecule[cont_point..].chars())
					.collect();
				replaced_molecules.push(string);

				cur = cont_point;
			}

			replaced_molecules
		})
		.collect::<aoc::Set<_>>();

	println!("part 1: number of unique molecules, replace once: {}", replaced_molecules.len());
}

#[derive(pest_derive::Parser)]
#[grammar = "parsers/y2015-d19.pest"]
pub struct InputParser;

trait PairExts {
	fn into_replacer(self) -> (String, String);
}

impl<'h> PairExts for pest::iterators::Pair<'h, Rule> {
	fn into_replacer(self) -> (String, String) {
		let mut inner = self.into_inner();

		let src = inner.next().unwrap().as_str().into();
		let to = inner.next().unwrap().as_str().into();

		(src, to)
	}
}
