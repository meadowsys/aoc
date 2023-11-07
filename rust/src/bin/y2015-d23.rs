use pest::Parser as _;

fn main() {
	let input_str = aoc::get_input!();
	let mut input = InputParser::parse(Rule::file, &input_str)
		.expect("input failed to parse");

	let file = input.next().unwrap();
	assert!(matches!(file.as_rule(), Rule::file));

	let instructions = file.into_inner()
		.take_while(|l| !matches!(l.as_rule(), Rule::EOI))
		.zip(1isize..)
		.map(|(line, instruction_pos)| match line.as_rule() {
			Rule::hlf => { (instruction_pos, line.into_hlf()) }
			Rule::tpl => { (instruction_pos, line.into_tpl()) }
			Rule::inc => { (instruction_pos, line.into_inc()) }
			Rule::jmp => { (instruction_pos, line.into_jmp()) }
			Rule::jie => { (instruction_pos, line.into_jie()) }
			Rule::jio => { (instruction_pos, line.into_jio()) }
			_ => { unreachable!() }
		})
		.collect::<aoc::Map<_, _>>();

	let mut pos = 1isize;
	let mut registers = aoc::map!();
	while let Some(instruction) = instructions.get(&pos) {
		let jump_instruction = instruction.operate_on(&mut registers);
		pos += jump_instruction;
	}

	println!("part 1: register b: {}", registers.get("b").unwrap());
}

#[derive(pest_derive::Parser)]
#[grammar = "parsers/y2015-d23.pest"]
pub struct InputParser;

enum Instruction {
	Hlf(String),
	Tpl(String),
	Inc(String),
	Jmp(isize),
	Jie(String, isize),
	Jio(String, isize)
}

impl Instruction {
	fn operate_on(&self, registers: &mut aoc::Map<String, usize>) -> isize {
		match self {
			Instruction::Hlf(r) => {
				let register = *registers.get(r)
					.unwrap_or(&0);
				registers.insert(r.clone(), register / 2);
				1
			}
			Instruction::Tpl(r) => {
				let register = *registers.get(r)
					.unwrap_or(&0);
				registers.insert(r.clone(), register * 3);
				1
			}
			Instruction::Inc(r) => {
				let register = *registers.get(r)
					.unwrap_or(&0);
				registers.insert(r.clone(), register + 1);
				1
			}
			Instruction::Jmp(ji) => {
				*ji
			}
			Instruction::Jie(r, ji) => {
				let register = *registers.get(r)
					.unwrap_or(&0);

				if register % 2 == 0 {
					*ji
				} else {
					1
				}
			}
			Instruction::Jio(r, ji) => {
				let register = *registers.get(r)
					.unwrap_or(&0);

				if register == 1 {
					*ji
				} else {
					1
				}
			}
		}
	}
}

trait PairExts {
	fn into_hlf(self) -> Instruction;
	fn into_tpl(self) -> Instruction;
	fn into_inc(self) -> Instruction;
	fn into_jmp(self) -> Instruction;
	fn into_jie(self) -> Instruction;
	fn into_jio(self) -> Instruction;
}

impl<'h> PairExts for pest::iterators::Pair<'h, Rule> {
	fn into_hlf(self) -> Instruction {
		let register = self.into_inner().next_register();
		Instruction::Hlf(register)
	}

	fn into_tpl(self) -> Instruction {
		let register = self.into_inner().next_register();
		Instruction::Tpl(register)
	}

	fn into_inc(self) -> Instruction {
		let register = self.into_inner().next_register();
		Instruction::Inc(register)
	}

	fn into_jmp(self) -> Instruction {
		let jump_instruction = self.into_inner().next_jump_instruction();
		Instruction::Jmp(jump_instruction)
	}

	fn into_jie(self) -> Instruction {
		let mut inner = self.into_inner();
		let register = inner.next_register();
		let jump_instruction = inner.next_jump_instruction();
		Instruction::Jie(register, jump_instruction)
	}

	fn into_jio(self) -> Instruction {
		let mut inner = self.into_inner();
		let register = inner.next_register();
		let jump_instruction = inner.next_jump_instruction();
		Instruction::Jio(register, jump_instruction)
	}
}

trait PairsExts {
	fn next_register(&mut self) -> String;
	fn next_jump_instruction(&mut self) -> isize;
}

impl<'h> PairsExts for pest::iterators::Pairs<'h, Rule> {
	fn next_register(&mut self) -> String {
		let register = self.next().unwrap();
		register.as_str().to_string()
	}

	fn next_jump_instruction(&mut self) -> isize {
		let mut inner = self.next().unwrap().into_inner();

		let sign = match inner.next().unwrap().as_str() {
			"+" => { 1isize }
			"-" => { -1isize }
			_ => { unreachable!() }
		};

		let jump_amount = inner.next().unwrap().as_str().parse::<isize>().unwrap();

		sign * jump_amount
	}
}
