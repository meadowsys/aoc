use fancy_regex::Regex;

fn main() {
	let input_str = aoc::get_input!();

	let code_size = input_str.trim()
		.split('\n')
		.map(str::len)
		.sum::<usize>();

	let regex_to_unescape = Regex::new("\\\\\\\\|\\\\\"|\\\\x[0-9a-fA-F]{2}").unwrap();
	let mem_size = input_str.trim()
		.split('\n')
		.map(|s| &s[1..s.len() - 1])
		.map(|s| regex_to_unescape.replace_all(s, "b"))
		.map(|s| s.len())
		.sum::<usize>();

	let difference = code_size - mem_size;

	println!("part 1: code size: {code_size}");
	println!("        memory size: {mem_size}");
	println!("        difference: {difference}");

	let to_escape_regex = Regex::new("(?<uwu>\\\\|\")").unwrap();
	let mem_size = input_str.trim()
		.split('\n')
		.map(|s| to_escape_regex.replace_all(s, "\\$uwu"))
		.map(|s| s.len() + 2)
		.sum::<usize>();

	let difference = mem_size - code_size;

	println!("part 2: code size: {code_size}");
	println!("        memory size: {mem_size}");
	println!("        difference: {difference}");
}
