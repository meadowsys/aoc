fn main() {
	let input_str = aoc::get_input!();
	let input_str = input_str.trim();

	let floor = input_str.chars()
		.fold(0isize, |acc, curr| acc + get_increment(curr));

	println!("part 1: floor {floor}");

	let mut current_floor = 0isize;
	for (direction, i) in input_str.chars().zip(1..) {
		current_floor += get_increment(direction);
		if current_floor == -1 {
			println!("part 2: step {i}");
			break
		}
	}
}

fn get_increment(c: char) -> isize {
	match c {
		'(' => { 1 }
		')' => { -1 }
		_ => { unreachable!() }
	}
}
