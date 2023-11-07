fn main() {
	let input_str = aoc::get_input!();

	let lines = input_str.trim()
		.lines()
		.map(|l| l.trim())
		.collect::<Vec<_>>();

	let code = process_lines_with(&lines, map_char_keypad_normal);
	println!("part 1: the code: {code}");

	let code = process_lines_with(&lines, map_char_keypad_weird);
	println!("part 1: the code with weird keypad: {code}");
}

fn process_lines_with(lines: &[&str], f: impl Fn(u8, u8) -> u8) -> String {
	let mut code = String::with_capacity(lines.len());
	let mut previous_digit = b'5';

	for line in lines {
		let mut current = previous_digit;

		for direction in line.chars() {
			current = f(current, direction as u8);
		}

		code.push(current as char);
		previous_digit = current;
	}

	code
}

#[allow(clippy::manual_range_patterns)]
fn map_char_keypad_normal(digit: u8, direction: u8) -> u8 {
	match direction {
		b'U' => match digit {
			b'1' | b'2' | b'3' => { digit }
			b'4' | b'5' | b'6' |
			b'7' | b'8' | b'9'
			=> { digit - 3 }
			_ => { unreachable!() }
		}

		b'R' => match digit {
			b'3' | b'6' | b'9' => { digit }
			b'1' | b'2' |
			b'4' | b'5' |
			b'7' | b'8'
			=> { digit + 1 }
			_ => { unreachable!() }
		}

		b'D' => match digit {
			b'7' | b'8' | b'9' => { digit }
			b'1' | b'2' | b'3' |
			b'4' | b'5' | b'6'
			=> { digit + 3 }
			_ => { unreachable!() }
		}

		b'L' => match digit {
			b'1' | b'4' | b'7' => { digit }
			b'2' | b'3' |
			b'5' | b'6' |
			b'8' | b'9'
			=> { digit - 1 }
			_ => { unreachable!() }
		}

		_ => { unreachable!() }
	}
}

#[allow(clippy::manual_range_patterns)]
fn map_char_keypad_weird(digit: u8, direction: u8) -> u8 {
	match direction {
		b'U' => match digit {
			b'1' => { b'1' }
			b'2' => { b'2' }
			b'3' => { b'1' }
			b'4' => { b'4' }
			b'5' => { b'5' }
			b'6' => { b'2' }
			b'7' => { b'3' }
			b'8' => { b'4' }
			b'9' => { b'9' }
			b'A' => { b'6' }
			b'B' => { b'7' }
			b'C' => { b'8' }
			b'D' => { b'B' }
			_ => { unreachable!() }
		}
		b'R' => match digit {
			b'1' => { b'1' }
			b'2' => { b'3' }
			b'3' => { b'4' }
			b'4' => { b'4' }
			b'5' => { b'6' }
			b'6' => { b'7' }
			b'7' => { b'8' }
			b'8' => { b'9' }
			b'9' => { b'9' }
			b'A' => { b'B' }
			b'B' => { b'C' }
			b'C' => { b'C' }
			b'D' => { b'D' }
			_ => { unreachable!() }
		}
		b'D' => match digit {
			b'1' => { b'3' }
			b'2' => { b'6' }
			b'3' => { b'7' }
			b'4' => { b'8' }
			b'5' => { b'5' }
			b'6' => { b'A' }
			b'7' => { b'B' }
			b'8' => { b'C' }
			b'9' => { b'9' }
			b'A' => { b'A' }
			b'B' => { b'D' }
			b'C' => { b'C' }
			b'D' => { b'D' }
			_ => { unreachable!() }
		}
		b'L' => match digit {
			b'1' => { b'1' }
			b'2' => { b'2' }
			b'3' => { b'2' }
			b'4' => { b'3' }
			b'5' => { b'5' }
			b'6' => { b'5' }
			b'7' => { b'6' }
			b'8' => { b'7' }
			b'9' => { b'8' }
			b'A' => { b'A' }
			b'B' => { b'A' }
			b'C' => { b'B' }
			b'D' => { b'D' }
			_ => { unreachable!() }
		}

		_ => { unreachable!() }
	}
}
