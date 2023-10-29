use pest::Parser as _;

fn main() {
	let input_str = aoc::get_input!();
	let mut input = InputParser::parse(Rule::file, &input_str)
		.expect("input failed to parse");

	let file = input.next().unwrap();
	assert!(matches!(file.as_rule(), Rule::file));

	let mut ingredients = Vec::new();
	let max_tsps = 100;

	for line in file.into_inner() {
		if matches!(line.as_rule(), Rule::EOI) { break }

		ingredients.push(line.into_cookie_ingredient());
	}

	let split_ingredients = (&*ingredients).split_ingredient_amounts(max_tsps);
	assert!(split_ingredients.iter().all(|q| q.iter().map(|i| i.0).sum::<usize>() == 100));

	let max_score = split_ingredients.iter()
		.map(|i| i.get_cookie_total_score())
		.max()
		.unwrap();

	println!("part 1: max score: {max_score}");
}

#[derive(pest_derive::Parser)]
#[grammar = "parsers/y2015-d15.pest"]
pub struct InputParser;

#[derive(Clone, Debug)]
struct CookieIngredient {
	name: String,
	scores: Scores
}

#[derive(Clone, Debug)]
struct Scores {
	capacity: isize,
	durability: isize,
	flavour: isize,
	texture: isize,
	calories: isize
}

trait PairExts {
	fn into_cookie_ingredient(self) -> CookieIngredient;

	fn into_name(self) -> String;
	fn into_capacity(self) -> isize;
	fn into_durability(self) -> isize;
	fn into_flavour(self) -> isize;
	fn into_texture(self) -> isize;
	fn into_calories(self) -> isize;
}

impl<'h> PairExts for pest::iterators::Pair<'h, Rule> {
	fn into_cookie_ingredient(self) -> CookieIngredient {
		let mut inner = self.into_inner();

		let name = inner.next().unwrap().into_name();
		let capacity = inner.next().unwrap().into_capacity();
		let durability = inner.next().unwrap().into_durability();
		let flavour = inner.next().unwrap().into_flavour();
		let texture = inner.next().unwrap().into_texture();
		let calories = inner.next().unwrap().into_calories();

		assert!(inner.next().is_none());

		let scores = Scores { capacity, durability, flavour, texture, calories };
		CookieIngredient { name, scores }
	}

	#[inline]
	fn into_name(self) -> String {
		assert!(matches!(self.as_rule(), Rule::name));
		self.as_span().as_str().into()
	}

	#[inline]
	fn into_capacity(self) -> isize {
		assert!(matches!(self.as_rule(), Rule::capacity));
		self.as_span().as_str().parse().unwrap()
	}

	#[inline]
	fn into_durability(self) -> isize {
		assert!(matches!(self.as_rule(), Rule::durability));
		self.as_span().as_str().parse().unwrap()
	}

	#[inline]
	fn into_flavour(self) -> isize {
		assert!(matches!(self.as_rule(), Rule::flavor));
		self.as_span().as_str().parse().unwrap()
	}

	#[inline]
	fn into_texture(self) -> isize {
		assert!(matches!(self.as_rule(), Rule::texture));
		self.as_span().as_str().parse().unwrap()
	}

	#[inline]
	fn into_calories(self) -> isize {
		assert!(matches!(self.as_rule(), Rule::calories));
		self.as_span().as_str().parse().unwrap()
	}
}

trait CookieIngredientsExts {
	fn split_ingredient_amounts(self, max_tsps: usize) -> Vec<Vec<(usize, CookieIngredient)>>;
}

impl CookieIngredientsExts for &[CookieIngredient] {
	fn split_ingredient_amounts(self, max_tsps: usize) -> Vec<Vec<(usize, CookieIngredient)>> {
		let [ingredient, ingredients @ ..] = self else {
			panic!("CookieIngredientsExts::get_cookie_qualities can only be called on slices with at least one element")
		};

		if ingredients.is_empty() {
			vec![vec![(max_tsps, ingredient.clone())]]
		} else {
			(0..=max_tsps)
				.flat_map(|i| {
					let remaining_tsps = max_tsps - i;
					let mut qualities = ingredients.split_ingredient_amounts(remaining_tsps);
					qualities.iter_mut()
						.for_each(|q| q.push((i, ingredient.clone())));
					qualities
				})
				.collect()
		}
	}
}

trait CookieIngredientQualitiesExts {
	fn get_cookie_total_score(self) -> usize;
}

impl CookieIngredientQualitiesExts for &[(usize, CookieIngredient)] {
	fn get_cookie_total_score(self) -> usize {
		let Scores { capacity, durability, flavour, texture, calories: _ } = self.iter()
			.map(|(amount, ingredient)| {
				let Scores { capacity, durability, flavour, texture, calories } = ingredient.scores;
				let amount = *amount as isize;
				let capacity = capacity * amount;
				let durability = durability * amount;
				let flavour = flavour * amount;
				let texture = texture * amount;
				Scores { capacity, durability, flavour, texture, calories }
			})
			.reduce(|s1, s2| {
				let capacity = s1.capacity + s2.capacity;
				let durability = s1.durability + s2.durability;
				let flavour = s1.flavour + s2.flavour;
				let texture = s1.texture + s2.texture;
				let calories = s1.calories + s2.calories;
				Scores { capacity, durability, flavour, texture, calories }
			})
			.unwrap();

		let capacity = capacity.max(0) as usize;
		let durability = durability.max(0) as usize;
		let flavour = flavour.max(0) as usize;
		let texture = texture.max(0) as usize;

		capacity * durability * flavour * texture
	}
}
