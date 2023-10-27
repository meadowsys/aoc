fn main() {
	let input_str = aoc::get_input!();
	let input_str = input_str.trim();

	let floor = input_str.chars().fold(0isize, |acc, curr| {
		match curr {
			'(' => { acc + 1 }
			')' => { acc - 1 }
			_ => { unreachable!() }
		}
	});

	println!("floor {floor}");
}
