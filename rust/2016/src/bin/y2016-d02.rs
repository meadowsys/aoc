fn main() {
	let input_str = aoc::get_input!();

	let lines = input_str.trim()
		.lines()
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let mut code = String::with_capacity(lines.len());
	let mut previous_digit = 5;

	for line in lines {
		let mut current = previous_digit;

		for direction in line.chars() {
			current = map_char(current, direction as u8);
		}

		code.push_str(&current.to_string());
		previous_digit = current;
	}

	println!("part 1: the code: {code}");
}

#[allow(clippy::manual_range_patterns)]
fn map_char(digit: u8, direction: u8) -> u8 {
	match direction {
		b'U' => match digit {
			1 | 2 | 3 => { digit }
			4 | 5 | 6 |
			7 | 8 | 9
			=> { digit - 3 }
			_ => { unreachable!() }
		}

		b'R' => match digit {
			3 | 6 | 9 => { digit }
			1 | 2 |
			4 | 5 |
			7 | 8
			=> { digit + 1 }
			_ => { unreachable!() }
		}

		b'D' => match digit {
			7 | 8 | 9 => { digit }
			1 | 2 | 3 |
			4 | 5 | 6
			=> { digit + 3 }
			_ => { unreachable!() }
		}

		b'L' => match digit {
			1 | 4 | 7 => { digit }
			2 | 3 |
			5 | 6 |
			8 | 9
			=> { digit - 1 }
			_ => { unreachable!() }
		}

		_ => { unreachable!() }
	}
}
