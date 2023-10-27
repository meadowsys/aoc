use pest::Parser as _;

fn main() {
	let input_str = aoc::get_input!();
	let mut input = InputParser::parse(Rule::file, &input_str)
		.expect("input failed to parse");

	let file = input.next().unwrap();
	assert!(matches!(file.as_rule(), Rule::file));

	let distances = {
		let mut distances = aoc::new_map();
		for line in file.into_inner() {
			if matches!(line.as_rule(), Rule::EOI) { break }

			let (city1, city2, distance) = line.into_cities();
			assert!(distances.insert((city1, city2), distance).is_none());
		}
		distances
	};

	let cities = distances.keys()
		.flat_map(|(c1, c2)| [c1.clone(), c2.clone()])
		.collect::<aoc::Set<_>>();

	let mut cities_chains = get_cities_chains(cities.clone())
		.into_iter()
		.map(|chain| {
			let total_distance = chain.windows(2)
				.map(|cities| {
					let mut cities = cities.iter();
					let city1 = cities.next().unwrap();
					let city2 = cities.next().unwrap();

					let (city1, city2) = sort_cities(city1, city2);
					distances.get(&(city1.clone(), city2.clone())).unwrap()
				})
				.sum::<usize>();
			(chain, total_distance)
		})
		.collect::<Vec<_>>();
	cities_chains.sort_unstable_by_key(|d| d.1);

	let shortest = cities_chains.first().unwrap().1;
	println!("part 1: shortest distance: {shortest}");

	let shortest = cities_chains.last().unwrap().1;
	println!("part 2: longest distance: {shortest}");
}

#[derive(pest_derive::Parser)]
#[grammar = "parsers/y2015-d09.pest"]
pub struct InputParser;

trait PairExts {
	fn into_cities(self) -> (String, String, usize);
}

impl<'h> PairExts for pest::iterators::Pair<'h, Rule> {
	fn into_cities(self) -> (String, String, usize) {
		let mut inner = self.into_inner();

		let city1 = inner.next().unwrap().as_span().as_str().into();
		let city2 = inner.next().unwrap().as_span().as_str().into();
		let (city1, city2) = sort_cities(city1, city2);

		let distance = inner.next().unwrap().as_span().as_str().parse().unwrap();
		(city1, city2, distance)
	}
}

#[inline]
fn sort_cities<T: AsRef<str>>(city1: T, city2: T) -> (T, T) {
	if city1.as_ref() > city2.as_ref() {
		(city2, city1)
	} else {
		(city1, city2)
	}
}

fn get_cities_chains(cities: aoc::Set<String>) -> aoc::Set<Vec<String>> {
	fn get_cities_chains_inner(cities: Vec<String>) -> Vec<Vec<String>> {
		if cities.len() == 1 {
			let city = cities.into_iter().next().unwrap();
			return vec![vec![city]]
		}

		cities.iter()
			.flat_map(|city| {
				let cities = cities.iter()
					.filter(|c| *c != city)
					.cloned()
					.collect::<Vec<_>>();

				let mut subchains = get_cities_chains_inner(cities);
				subchains.iter_mut()
					.for_each(|subchain| subchain.push(city.clone()));

				subchains
			})
			.collect::<Vec<_>>()
	}

	let vec = get_cities_chains_inner(cities.into_iter().collect());
	let vec_len = vec.len();

	let set = vec.into_iter().collect::<aoc::Set<_>>();
	let set_len = set.len();

	assert_eq!(vec_len, set_len);

	set
}
