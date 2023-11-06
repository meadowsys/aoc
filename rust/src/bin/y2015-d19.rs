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

	let sorted_replacers = {
		let mut temp = replacers.iter()
			.cloned()
			.collect::<Vec<_>>();
		temp.sort_unstable_by_key(|r| std::cmp::Reverse((r.1.len(), r.0.len())));
		temp
	};

	let steps = get_to_e(molecule, &sorted_replacers).unwrap();
	eprintln!("part 2: steps to medicine: {steps}");
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

#[inline]
fn get_to_e(molecule: String, replacers: &Vec<(String, String)>) -> Option<usize> {
	fn get_to_e_inner(
		molecule: String,
		replacers: &Vec<(String, String)>,
		step: usize
	) -> Option<usize> {
		if molecule == "e" { return Some(step) }

		for (i, _) in molecule.char_indices().rev() {
			let start = &molecule[..i];
			let end = &molecule[i..];

			for (rep, pat) in replacers {
				if !end.starts_with(pat) { continue }

				let end = &end[pat.len()..];
				let new_molecule = start.chars()
					.chain(rep.chars())
					.chain(end.chars())
					.collect::<String>();

				let res = get_to_e_inner(new_molecule, replacers, step + 1);
				if res.is_some() { return res }
			}
		}

		None
	}

	get_to_e_inner(molecule, replacers, 0)
}
