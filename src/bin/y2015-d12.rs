use serde_json::Value;

fn main() {
	let input_str = aoc::get_input!();

	let value = serde_json::from_str::<Value>(&input_str).unwrap();

	let sum = process_value_sum(value);
	println!("part 1: sum of all numbers: {sum}");
}


fn process_value_sum(value: Value) -> isize {
	match value {
		Value::Number(num) => { num.as_i64().unwrap() as isize }
		Value::Array(array) => {
			array.into_iter()
				.map(process_value_sum)
				.sum()
		}
		Value::Object(obj) => {
			obj.into_iter()
				.map(|(_, val)| val)
				.map(process_value_sum)
				.sum()
		}

		Value::Null | Value::Bool(_) | Value::String(_) => { 0 }
	}
}
