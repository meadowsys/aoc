use fancy_regex::Regex;

fn main() {
	let input_str = aoc::get_input!();
	let input = input_str.trim();

	let regex = Regex::new(r"(?:(\d)\1*)").unwrap();
	let iterations = 40;

	let mut results = Vec::<String>::with_capacity(iterations);
	for _ in 0..iterations {
		let prev = results.last()
			.map(|s| s.as_str())
			.unwrap_or_else(|| input);
		let result = look_and_say(prev, &regex);
		results.push(result);
	}

	println!("part 1: length after 40 iters: {}", results.last().unwrap().len());

	if aoc::allow_fun!() {
		let str_size = results.iter().map(String::len).sum::<usize>();
		let mem_used = results.iter().map(String::capacity).sum::<usize>();
		println!("just for fun: actual str size: {str_size}");
		println!("              memory used (capacity): {mem_used}");
		println!("              memory wasted: {}", mem_used - str_size);
	}
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
