use serde_json::Value;

fn main() {
	let input_str = aoc::get_input!();

	let value = serde_json::from_str::<Value>(&input_str).unwrap();

	let sum = process_value_sum(&value, false);
	println!("part 1: sum of all numbers: {sum}");

	let sum = process_value_sum(&value, true);
	println!("part 2: sum, filtering objs with value \"red\": {sum}");
}


fn process_value_sum(value: &Value, filter_red: bool) -> isize {
	match value {
		Value::Number(num) => { num.as_i64().unwrap() as isize }
		Value::Array(array) => {
			array.iter()
				.map(|i| process_value_sum(i, filter_red))
				.sum()
		}
		Value::Object(obj) => {
			if filter_red && obj.values().any(|v| if let Value::String(s) = v { s == "red" } else { false }) {
				0
			} else {
				obj.iter()
					.map(|(_, val)| val)
					.map(|i| process_value_sum(i, filter_red))
					.sum()
			}
		}

		Value::Null | Value::Bool(_) | Value::String(_) => { 0 }
	}
}
