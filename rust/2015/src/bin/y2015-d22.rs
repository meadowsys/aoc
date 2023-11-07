use pest::Parser as _;
use std::ops::ControlFlow;
use std::thread::spawn;

fn main() {
	let input_str = aoc::get_input!();
	let mut input = InputParser::parse(Rule::file, &input_str)
		.expect("input failed to parse");

	let file = input.next().unwrap();
	assert!(matches!(file.as_rule(), Rule::file));

	let mut file = file.into_inner();

	let boss_hit_points = file.next().unwrap().as_str().parse::<isize>().unwrap();
	let boss_damage = file.next().unwrap().as_str().parse::<isize>().unwrap();

	assert!(matches!(file.next().unwrap().as_rule(), Rule::EOI));
	assert!(file.next().is_none());

	let player_hit_points = 50;
	let player_mana = 500;

	let game = Game {
		active_effects: aoc::map!(),
		boss_hit_points,
		boss_damage,
		player_hit_points,
		player_mana,
		player_armour: 0,
		total_mana_spent: 0
	};

	let game2 = game.clone();
	let least_mana = spawn(|| {
		game2.least_mana_still_win(false).unwrap()
	});

	let least_mana_hard = spawn(|| {
		game.least_mana_still_win(true).unwrap()
	});

	let least_mana = least_mana.join().unwrap();
	let least_mana_hard = least_mana_hard.join().unwrap();

	println!("part 1: least mana to win the game: {least_mana}");
	println!("part 2: least mana to win (hard mode): {least_mana_hard}");
}

#[derive(pest_derive::Parser)]
#[grammar = "parsers/y2015-d22.pest"]
pub struct InputParser;

#[derive(Clone, Debug)]
struct Game {
	active_effects: aoc::Map<Effect, usize>,

	boss_hit_points: isize,
	boss_damage: isize,
	player_hit_points: isize,
	player_mana: isize,
	player_armour: isize,

	total_mana_spent: isize
}

enum Character {
	Player,
	Boss
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Effect {
	MagicMissile,
	Drain,
	Shield,
	Poison,
	Recharge
}

type GameResult = ControlFlow<(Character, Game), Game>;

impl Game {
	fn subtract_player_mana(mut self, mana: isize) -> GameResult {
		self.player_mana -= mana;

		if self.player_mana <= 0 {
			GameResult::Break((Character::Boss, self))
		} else {
			self.total_mana_spent += mana;
			GameResult::Continue(self)
		}
	}

	fn subtract_player_hit_points(mut self, hitpoints: isize) -> GameResult {
		self.player_hit_points -= hitpoints;

		if self.player_hit_points <= 0 {
			GameResult::Break((Character::Boss, self))
		} else {
			GameResult::Continue(self)
		}
	}

	fn subtract_boss_hit_points(mut self, hitpoints: isize) -> GameResult {
		self.boss_hit_points -= hitpoints;
		if self.boss_hit_points <= 0 {
			GameResult::Break((Character::Player, self))
		} else {
			GameResult::Continue(self)
		}
	}

	fn boss_attack(mut self) -> GameResult {
		let boss_damage = 1.max(self.boss_damage - self.player_armour);
		self.player_hit_points -= boss_damage;

		if self.player_hit_points <= 0 {
			GameResult::Break((Character::Boss, self))
		} else {
			GameResult::Continue(self)
		}
	}

	fn player_cast_spell(mut self, effect: Effect) -> GameResult {
		match effect {
			Effect::MagicMissile => {
				self = self.subtract_player_mana(53)?;
				self = self.subtract_boss_hit_points(4)?;
			}
			Effect::Drain => {
				self = self.subtract_player_mana(73)?;
				self = self.subtract_boss_hit_points(2)?;
				self.player_hit_points += 2;
			}
			Effect::Shield => {
				self = self.subtract_player_mana(113)?;
				self.player_armour += 7;
				self.active_effects.insert(Effect::Shield, 6);
			}
			Effect::Poison => {
				self = self.subtract_player_mana(173)?;
				self.active_effects.insert(Effect::Poison, 6);
			}
			Effect::Recharge => {
				self = self.subtract_player_mana(229)?;
				self.active_effects.insert(Effect::Recharge, 5);
			}
		}

		GameResult::Continue(self)
	}

	fn tick_effects(mut self) -> GameResult {
		let active_effects = self.active_effects.clone();
		self.active_effects.clear();

		for (effect, turns) in active_effects.into_iter() {
			if turns > 0 {
				let turns = turns - 1;

				match effect {
					Effect::Shield => {
						if turns == 0 {
							self.player_armour -= 7;
						}
					}
					Effect::Poison => {
						self = self.subtract_boss_hit_points(3)?;
					}
					Effect::Recharge => {
						self.player_mana += 101;
					}
					_ => { unreachable!() }
				}

				if turns > 0 {
					self.active_effects.insert(effect, turns);
				}
			}
		}

		GameResult::Continue(self)
	}

	fn least_mana_still_win(self, hard_mode: bool) -> Option<isize> {
		const ALL_EFFECTS: &[Effect] = &[
			Effect::MagicMissile,
			Effect::Drain,
			Effect::Shield,
			Effect::Poison,
			Effect::Recharge
		];

		ALL_EFFECTS.iter()
			.filter(|e| if let Some(turns) = self.active_effects.get(*e) {
				// first pre-player-turn tick will make this expire
				// so it will be available to cast again
				*turns <= 1
			} else {
				true
			})
			.cloned()
			.zip([self.clone()].iter().cycle().cloned())
			.map(|(effect, mut game)| {
				if hard_mode { game = game.subtract_player_hit_points(1)? }

				// player turn
				game = game.tick_effects()?;
				game = game.player_cast_spell(effect)?;

				// boss turn
				game = game.tick_effects()?;
				game = game.boss_attack()?;

				GameResult::Continue(game)
			})
			.filter_map(|res| match res {
				GameResult::Continue(game) => {
					game.least_mana_still_win(hard_mode)
				}
				GameResult::Break((Character::Player, game)) => {
					Some(game.total_mana_spent)
				}
				_ => { None }
			})
			.min()
	}
}
