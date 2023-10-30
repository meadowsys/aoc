use pest::Parser as _;

fn main() {
	let input_str = aoc::get_input!();
	let mut input = InputParser::parse(Rule::file, &input_str)
		.expect("input failed to parse");

	let file = input.next().unwrap();
	assert!(matches!(file.as_rule(), Rule::file));

	let relations = file.into_inner()
		.take_while(|l| !matches!(l.as_rule(), Rule::EOI))
		.map(|l| l.into_happiness_relation())
		.collect::<aoc::Map<_, _>>();

	let people = relations.keys()
		.flat_map(|(p1, p2)| [p1.clone(), p2.clone()])
		.collect::<aoc::Set<_>>();

	let mut orderings = get_people_ordering(people)
		.into_iter()
		.map(|order| {
			let mut total_happiness = order.windows(2)
				.flat_map(|people| {
					let [p1, p2] = people else { unreachable!() };
					let (r1, r2) = get_both_relations(p1.into(), p2.into(), &relations);
					[r1, r2]
				})
				.sum::<isize>();

			let (p1, p2) = (order.first().unwrap(), order.last().unwrap());
			let (r1, r2) = get_both_relations(p1.into(), p2.into(), &relations);
			total_happiness += r1 + r2;

			(order, total_happiness)
		})
		.collect::<Vec<_>>();
	orderings.sort_unstable_by_key(|o| o.1);

	let greatest_happiness = orderings.last().unwrap().1;
	println!("part 1: greatest happiness: {greatest_happiness}");
}

#[derive(pest_derive::Parser)]
#[grammar = "parsers/y2015-d13.pest"]
pub struct InputParser;

trait PairExts {
	fn into_happiness_relation(self) -> ((String, String), isize);
	fn into_gain_multiplier(self) -> isize;
}

impl<'h> PairExts for pest::iterators::Pair<'h, Rule> {
	fn into_happiness_relation(self) -> ((String, String), isize) {
		let mut inner = self.into_inner();

		let name1 = inner.next().unwrap().as_span().as_str().into();
		let happiness_multiplier = inner.next().unwrap().into_gain_multiplier();
		let happiness = inner.next().unwrap().as_span().as_str().parse::<isize>().unwrap();
		let happiness = happiness * happiness_multiplier;
		let name2 = inner.next().unwrap().as_span().as_str().into();

		((name1, name2), happiness)
	}

	fn into_gain_multiplier(self) -> isize {
		match self.as_rule() {
			Rule::gain => { 1 }
			Rule::lose => { -1 }
			_ => { unreachable!() }
		}
	}
}

fn get_people_ordering(people: aoc::Set<String>) -> aoc::Set<Vec<String>> {
	fn get_people_ordering_inner(people: Vec<String>) -> Vec<Vec<String>> {
		if people.len() == 1 {
			vec![vec![people.into_iter().next().unwrap()]]
		} else {
			people.iter()
				.flat_map(|person| {
					let people = people.iter()
						.filter(|p| *p != person)
						.cloned()
						.collect();
					let mut subgroup = get_people_ordering_inner(people);
					subgroup.iter_mut().for_each(|g| g.push(person.clone()));
					subgroup
				})
				.collect()
		}
	}

	let vec = get_people_ordering_inner(people.into_iter().collect());
	let vec_len = vec.len();

	let set = vec.into_iter().collect::<aoc::Set<_>>();
	let set_len = set.len();

	assert_eq!(vec_len, set_len);

	set
}

fn get_both_relations(p1: String, p2: String, relations: &aoc::Map<(String, String), isize>) -> (isize, isize) {
	let r1 = relations.get(&(p1.clone(), p2.clone())).unwrap();
	let r2 = relations.get(&(p2, p1)).unwrap();
	(*r1, *r2)
}
