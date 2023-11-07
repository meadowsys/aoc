use std::thread::spawn;

fn main() {
	let input_str = aoc::get_input!();
	let input = input_str.trim();

	let target = input.parse::<usize>().unwrap();
	let max_house = target / 10;

	let part1 = spawn(move || {
		(1usize..=max_house)
			.find(|house| {
				let presents = (1..=*house)
					.map(|num| elf(num).presents_for(*house))
					.sum::<usize>();

				presents >= target
			})
			.unwrap_or(max_house)
	});

	let max_house = (target as f64 / 11.).ceil() as usize;

	let part2 = spawn(move || {
		(1usize..=max_house)
			.find(|house| {
				let presents = (1..=*house)
					.map(|num| elf(num).lazy_presents_for(*house))
					.sum::<usize>();

				presents >= target
			})
			.unwrap_or(max_house)
	});

	let part1 = part1.join().unwrap();
	let part2 = part2.join().unwrap();

	println!("part 1: house with at least {input} presents: {part1}");
	println!("part 2: house with at least {input} presents (elfs are lazy): {part2}");
}

#[repr(transparent)]
struct Elf(usize);

#[inline]
fn elf(num: usize) -> Elf {
	Elf(num)
}

impl Elf {
	#[inline]
	fn presents_for(self, house: usize) -> usize {
		if house % self.0 == 0 {
			self.0 * 10
		} else {
			0
		}
	}

	#[inline]
	fn lazy_presents_for(self, house: usize) -> usize {
		// let past_max = (house / self.0) > 50;
		let still_not_lazy = (50 * self.0) >= house;

		if still_not_lazy && house % self.0 == 0 {
			self.0 * 11
		} else {
			0
		}
	}
}
