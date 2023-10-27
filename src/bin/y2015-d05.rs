use fancy_regex::Regex;

fn main() {
	let input = aoc::get_input!();

	let regex1_3_vowels = Regex::new("^(?:[a-zA-Z]*?[aeiouAEIOU]){3,}?[a-zA-Z]*?$").unwrap();
	let regex2_double_letter = Regex::new("^[a-zA-Z]*?([a-zA-Z])\\1[a-zA-Z]*?$").unwrap();
	let regex3_no_forbidden_sequences = Regex::new("^((?!ab|cd|pq|xy).)*$").unwrap();

	let nice_strings_count = input.split('\n')
		.filter(|s| {
			let r1 = regex1_3_vowels.is_match(s).unwrap();
			let r2 = regex2_double_letter.is_match(s).unwrap();
			let r3 = regex3_no_forbidden_sequences.is_match(s).unwrap();

			r1 && r2 && r3
		})
		.count();

	println!("part 1: nice strings count: {nice_strings_count}");

	let regex4_nonoverlapping_letter_pairs = Regex::new("^[a-zA-Z]*?([a-zA-Z][a-zA-Z])[a-zA-Z]*?\\1[a-zA-Z]*?$").unwrap();
	let regex5_letter_repeat_with_letter_between = Regex::new("^[a-zA-Z]*?(?:([a-zA-Z])[a-zA-Z]\\1)[a-zA-Z]*?$").unwrap();

	let nice_strings_count = input.split('\n')
		.filter(|s| {
			let r1 = regex4_nonoverlapping_letter_pairs.is_match(s).unwrap();
			let r2 = regex5_letter_repeat_with_letter_between.is_match(s).unwrap();

			r1 && r2
		})
		.count();

	println!("part 2: nice strings count: {nice_strings_count}");
}
