use pest::Parser as _;

fn main() {
	let input_str = aoc::get_input!();
	let mut input = InputParser::parse(Rule::file, &input_str)
		.expect("input failed to parse");

	let file = input.next().unwrap();
	assert!(matches!(file.as_rule(), Rule::file));

	let mut wires = aoc::new_map::<String, WireSource>();

	for line in file.into_inner() {
		let line_rule = line.as_rule();
		let mut line = line.into_inner();

		match line_rule {
			Rule::value => {
				let (src, dest) = line.next_single_src();
				assert!(wires.insert(dest, WireSource::Value(src)).is_none());
			}

			Rule::and => {
				let (src1, src2, dest) = line.next_2_srcs();
				assert!(wires.insert(dest, WireSource::And(src1, src2)).is_none());
			}
			Rule::or => {
				let (src1, src2, dest) = line.next_2_srcs();
				assert!(wires.insert(dest, WireSource::Or(src1, src2)).is_none());
			}
			Rule::not => {
				let (src, dest) = line.next_single_src();
				assert!(wires.insert(dest, WireSource::Not(src)).is_none());
			}
			Rule::lshift => {
				let (src1, src2, dest) = line.next_2_srcs();
				assert!(wires.insert(dest, WireSource::LShift(src1, src2)).is_none());
			}
			Rule::rshift => {
				let (src1, src2, dest) = line.next_2_srcs();
				assert!(wires.insert(dest, WireSource::RShift(src1, src2)).is_none());
			}

			Rule::EOI => { break }
			_ => { unreachable!("{}", line) }
		}
	}

	assert!(input.next().is_none());

	println!("wire a: {}", compute_signal("a", &mut wires));
}

#[derive(pest_derive::Parser)]
#[grammar = "parsers/y2015-d07.pest"]
pub struct InputParser;

#[derive(Clone)]
enum ValueRef {
	Value(u16),
	Wire(String)
}

#[derive(Clone)]
enum WireSource {
	Value(ValueRef),
	And(ValueRef, ValueRef),
	Or(ValueRef, ValueRef),
	Not(ValueRef),
	LShift(ValueRef, ValueRef),
	RShift(ValueRef, ValueRef)
}

trait PairsExts {
	fn next_num_or_ident(&mut self) -> ValueRef;
	fn next_dest(&mut self) -> String;

	fn next_single_src(&mut self) -> (ValueRef, String);
	fn next_2_srcs(&mut self) -> (ValueRef, ValueRef, String);
}

impl<'h> PairsExts for pest::iterators::Pairs<'h, Rule> {
	#[inline]
	fn next_num_or_ident(&mut self) -> ValueRef {
		let next = self.next().unwrap();
		match next.as_rule() {
			Rule::number => { ValueRef::Value(next.as_span().as_str().parse().unwrap()) }
			Rule::ident => { ValueRef::Wire(next.as_span().as_str().into()) }
			_ => { unreachable!() }
		}
	}
	#[inline]
	fn next_dest(&mut self) -> String {
		let next = self.next().unwrap();
		if let Rule::ident = next.as_rule() {
			next.as_span().as_str().into()
		} else { unreachable!() }
	}

	#[inline]
	fn next_single_src(&mut self) -> (ValueRef, String) {
		let src = self.next_num_or_ident();
		let dest = self.next_dest();
		(src, dest)
	}
	#[inline]
	fn next_2_srcs(&mut self) -> (ValueRef, ValueRef, String) {
		let src1 = self.next_num_or_ident();
		let src2 = self.next_num_or_ident();
		let dest = self.next_dest();
		(src1, src2, dest)
	}
}

fn compute_signal(wire_name: &str, map: &mut aoc::Map<String, WireSource>) -> u16 {
	let wire = map.get(wire_name).unwrap().clone();

	let signal = match wire {
		WireSource::Value(val) => { get_value_from_ref(val, map) }

		WireSource::And(src1, src2) => {
			let src1 = get_value_from_ref(src1, map);
			let src2 = get_value_from_ref(src2, map);
			src1 & src2
		}

		WireSource::Or(src1, src2) => {
			let src1 = get_value_from_ref(src1, map);
			let src2 = get_value_from_ref(src2, map);
			src1 | src2
		}

		WireSource::Not(val) => {
			let val = get_value_from_ref(val, map);
			!val
		}

		WireSource::LShift(src1, src2) => {
			let src1 = get_value_from_ref(src1, map);
			let src2 = get_value_from_ref(src2, map);
			src1 << src2
		}

		WireSource::RShift(src1, src2) => {
			let src1 = get_value_from_ref(src1, map);
			let src2 = get_value_from_ref(src2, map);
			src1 >> src2
		}
	};

	map.insert(wire_name.into(), WireSource::Value(ValueRef::Value(signal)));
	signal
}

#[inline]
fn get_value_from_ref(val: ValueRef, map: &mut aoc::Map<String, WireSource>) -> u16 {
	match val {
		ValueRef::Value(val) => { val }
		ValueRef::Wire(wire_name) => { compute_signal(&wire_name, map) }
	}
}
