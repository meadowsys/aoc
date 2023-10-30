use pest::Parser as _;

fn main() {
	let input_str = aoc::get_input!();
	let mut input = InputParser::parse(Rule::file, &input_str)
		.expect("input failed to parse");

	let file = input.next().unwrap();
	assert!(matches!(file.as_rule(), Rule::file));

	let expected_children = 3;
	let expected_cats = 7;
	let expected_samoyeds = 2;
	let expected_pomeranians = 3;
	let expected_akitas = 0;
	let expected_vizslas = 0;
	let expected_goldfish = 5;
	let expected_trees = 3;
	let expected_cars = 2;
	let expected_perfumes = 1;

	let sue = file.clone().into_inner()
		.take_while(|sue| !matches!(sue.as_rule(), Rule::EOI))
		.map(|sue| sue.into_sue())
		.filter(|sue| if let Some(a) = sue.children { a == expected_children } else { true })
		.filter(|sue| if let Some(a) = sue.cats { a == expected_cats } else { true })
		.filter(|sue| if let Some(a) = sue.samoyeds { a == expected_samoyeds } else { true })
		.filter(|sue| if let Some(a) = sue.pomeranians { a == expected_pomeranians } else { true })
		.filter(|sue| if let Some(a) = sue.akitas { a == expected_akitas } else { true })
		.filter(|sue| if let Some(a) = sue.vizslas { a == expected_vizslas } else { true })
		.filter(|sue| if let Some(a) = sue.goldfish { a == expected_goldfish } else { true })
		.filter(|sue| if let Some(a) = sue.trees { a == expected_trees } else { true })
		.filter(|sue| if let Some(a) = sue.cars { a == expected_cars } else { true })
		.filter(|sue| if let Some(a) = sue.perfumes { a == expected_perfumes } else { true })
		.collect::<Vec<_>>();

	assert_eq!(sue.len(), 1);

	let sue = sue.into_iter().next().unwrap();
	println!("part 1: sue #{}", sue.num);

	let sue = file.clone().into_inner()
		.take_while(|sue| !matches!(sue.as_rule(), Rule::EOI))
		.map(|sue| sue.into_sue())
		.filter(|sue| if let Some(a) = sue.children { a == expected_children } else { true })
		.filter(|sue| if let Some(a) = sue.cats { a > expected_cats } else { true })
		.filter(|sue| if let Some(a) = sue.samoyeds { a == expected_samoyeds } else { true })
		.filter(|sue| if let Some(a) = sue.pomeranians { a < expected_pomeranians } else { true })
		.filter(|sue| if let Some(a) = sue.akitas { a == expected_akitas } else { true })
		.filter(|sue| if let Some(a) = sue.vizslas { a == expected_vizslas } else { true })
		.filter(|sue| if let Some(a) = sue.goldfish { a < expected_goldfish } else { true })
		.filter(|sue| if let Some(a) = sue.trees { a > expected_trees } else { true })
		.filter(|sue| if let Some(a) = sue.cars { a == expected_cars } else { true })
		.filter(|sue| if let Some(a) = sue.perfumes { a == expected_perfumes } else { true })
		.collect::<Vec<_>>();

	assert_eq!(sue.len(), 1);

	let sue = sue.into_iter().next().unwrap();
	println!("part 2: sue #{}", sue.num);
}

#[derive(pest_derive::Parser)]
#[grammar = "parsers/y2015-d16.pest"]
pub struct InputParser;

#[derive(Default)]
struct Sue {
	num: usize,
	children: Option<usize>,
	cats: Option<usize>,
	samoyeds: Option<usize>,
	pomeranians: Option<usize>,
	akitas: Option<usize>,
	vizslas: Option<usize>,
	goldfish: Option<usize>,
	trees: Option<usize>,
	cars: Option<usize>,
	perfumes: Option<usize>
}

impl Sue {
	fn new(num: usize) -> Self {
		Sue { num, ..Default::default() }
	}
}

trait PairExts {
	fn into_sue(self) -> Sue;

	fn into_suenum(self) -> usize;
	fn into_item(self, sue: &mut Sue);
}

impl<'h> PairExts for pest::iterators::Pair<'h, Rule> {
	fn into_sue(self) -> Sue {
		let mut inner = self.into_inner();

		let suenum = inner.next().unwrap().into_suenum();
		let mut sue = Sue::new(suenum);

		inner.for_each(|i| i.into_item(&mut sue));

		sue
	}

	#[inline]
	fn into_suenum(self) -> usize {
		self.as_span().as_str().parse().unwrap()
	}

	fn into_item(self, sue: &mut Sue) {
		let mut inner = self.into_inner();

		let name = inner.next().unwrap().as_span().as_str();
		let amount = inner.next().unwrap().as_span().as_str().parse().unwrap();

		match name {
			"children" => { sue.children = Some(amount) }
			"cats" => { sue.cats = Some(amount) }
			"samoyeds" => { sue.samoyeds = Some(amount) }
			"pomeranians" => { sue.pomeranians = Some(amount) }
			"akitas" => { sue.akitas = Some(amount) }
			"vizslas" => { sue.vizslas = Some(amount) }
			"goldfish" => { sue.goldfish = Some(amount) }
			"trees" => { sue.trees = Some(amount) }
			"cars" => { sue.cars = Some(amount) }
			"perfumes" => { sue.perfumes = Some(amount) }
			_ => { unreachable!() }
		}
	}
}
