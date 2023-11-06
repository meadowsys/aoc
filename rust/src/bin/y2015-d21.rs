use itertools::Itertools as _;
use pest::Parser as _;

fn main() {
	let input_str = aoc::get_input!();
	let mut input = InputParser::parse(Rule::file, &input_str)
		.expect("input failed to parse");

	let file = input.next().unwrap();
	assert!(matches!(file.as_rule(), Rule::file));

	let mut file = file.into_inner();

	let boss_hit_points = file.next().unwrap().as_str().parse::<isize>().unwrap();
	let boss_damage = file.next().unwrap().as_str().parse::<isize>().unwrap();
	let boss_armour = file.next().unwrap().as_str().parse::<isize>().unwrap();

	let player_hit_points = 100isize;

	assert!(matches!(file.next().unwrap().as_rule(), Rule::EOI));
	assert!(file.next().is_none());

	let amount_weapons = 1..=1;
	let amount_armour = 0..=1;
	let amount_rings = 0..=2;

	let mut least_cost = isize::MAX;

	for w in amount_weapons.clone() {
		for a in amount_armour.clone() {
			for r in amount_rings.clone() {
				let weapons = WEAPONS.iter().combinations(w);

				for w in weapons {
					let armour = ARMOUR.iter().combinations(a);

					for a in armour {
						let rings = RINGS.iter().combinations(r);

						for r in rings {
							let all_items = w.iter()
								.chain(a.iter())
								.chain(r.iter())
								.collect::<Vec<_>>();

							let player_damage = all_items.iter()
								.map(|i| i.damage)
								.sum::<isize>();
							let player_armour = all_items.iter()
								.map(|i| i.armour)
								.sum::<isize>();

							let game = Game {
								boss_hit_points,
								boss_damage,
								boss_armour,
								player_hit_points,
								player_damage,
								player_armour
							};
							let player_won = game.will_player_win();
							if player_won {
								let cost = all_items.iter()
									.map(|i| i.cost)
									.sum::<isize>();
								least_cost = least_cost.min(cost);
							}
						}
					}
				}
			}
		}
	}

	println!("part 1: least cost to win: {least_cost}");
}

#[derive(pest_derive::Parser)]
#[grammar = "parsers/y2015-d21.pest"]
pub struct InputParser;

struct Item {
	cost: isize,
	damage: isize,
	armour: isize
}

struct Game {
	boss_hit_points: isize,
	boss_damage: isize,
	boss_armour: isize,
	player_hit_points: isize,
	player_damage: isize,
	player_armour: isize
}

impl Game {
	fn will_player_win(self) -> bool {
		let Self {
			mut boss_hit_points,
			boss_damage,
			boss_armour,
			mut player_hit_points,
			player_damage,
			player_armour
		} = self;

		let (player_damage, boss_damage) = (
			1.max(player_damage - boss_armour),
			1.max(boss_damage - player_armour)
		);

		loop {
			// player's turn
			boss_hit_points -= player_damage;
			if boss_hit_points <= 0 { break true }

			// boss's turn
			player_hit_points -= boss_damage;
			if player_hit_points <= 0 { break false }
		}
	}
}

const WEAPONS: &[Item] = &[
	Item { cost: 8, damage: 4, armour: 0 },
	Item { cost: 10, damage: 5, armour: 0 },
	Item { cost: 25, damage: 6, armour: 0 },
	Item { cost: 40, damage: 7, armour: 0 },
	Item { cost: 74, damage: 8, armour: 0 }
];

const ARMOUR: &[Item] = &[
	Item { cost: 13, damage: 0, armour: 1 },
	Item { cost: 31, damage: 0, armour: 2 },
	Item { cost: 53, damage: 0, armour: 3 },
	Item { cost: 75, damage: 0, armour: 4 },
	Item { cost: 102, damage: 0, armour: 5 }
];

const RINGS: &[Item] = &[
	Item { cost: 25, damage: 1, armour: 0 },
	Item { cost: 50, damage: 2, armour: 0 },
	Item { cost: 100, damage: 3, armour: 0 },
	Item { cost: 20, damage: 0, armour: 1 },
	Item { cost: 40, damage: 0, armour: 2 },
	Item { cost: 80, damage: 0, armour: 3 }
];
