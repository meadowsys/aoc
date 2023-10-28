use fancy_regex::Regex;

fn main() {
	let input_str = aoc::get_input!();
	let input = input_str.trim();

	let regex = Regex::new(r"(?:(\d)\1*)").unwrap();

	let iterations = 40;
	let result = loop_look_and_say(input, &regex, iterations);
	println!(
		"part 1: length after {iterations} iterations: {}",
		result.len()
	);

	let more_iterations = 50;
	let result = loop_look_and_say(&result, &regex, more_iterations - iterations);
	println!(
		"part 2: length after {more_iterations} iterations: {}",
		result.len()
	);
}

fn look_and_say(input: &str, regex: &Regex) -> String {
	assert!(input.chars().all(char::is_numeric));

	let mut result = String::with_capacity(input.len() * 2);
	for capture in regex.captures_iter(input) {
		let capture = capture.unwrap();
		let capture = capture.get(0).unwrap().as_str();
		let len = capture.len();
		let char = capture.chars().next().unwrap();

		result.push_str(&len.to_string());
		result.push(char);
	}

	result
}

fn loop_look_and_say(input: &str, regex: &Regex, iterations: usize) -> String {
	if iterations == 0 {
		input.into()
	} else {
		look_and_say(&loop_look_and_say(input, regex, iterations - 1), regex)
	}
}
