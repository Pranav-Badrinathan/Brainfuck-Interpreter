use std::env;
use std::fs;
use std::io::Read;
use std::process;
use std::mem;

#[derive(Debug)]
enum Instruction {
    IncrementPointer,		// ">"
	DecrementPointer,		// "<"
	Add, 					// "+"
	Subtract, 				// "-"
	Read, 					// ","
	Write, 					// "."
	Loop(Vec<Instruction>),	// "[]" put the code inside a loop inside the Vec.
}

fn main() {
	// Get the CLI aruments the exe was called with.
	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		println!("Usage: bf <bf_file.bf>");
	}

	let file_contents: String;

	if let Some(file_path) = args.get(1) {	
		if let Ok(file_result) = fs::read_to_string(&file_path) {
		    file_contents = file_result;
		} else {
			eprintln!("File not found: Did you provide the correct path?");
			process::exit(1);
		}
		
		dbg!(&file_path);
	} else {
		eprintln!("Please provide a file to parse!");
		process::exit(1);
	}
	
	let instructions : Vec<Instruction> = lex_parse(&file_contents);
	// dbg!(&instructions);

	execute(&instructions);
}

fn lex_parse(source: &str) -> Vec<Instruction> {
	let mut instructs: Vec<Instruction> = Vec::new();
	let mut temp_stack = Vec::new();

	for token in source.chars() {
		match token {
			'>' => instructs.push(Instruction::IncrementPointer),
			'<' => instructs.push(Instruction::DecrementPointer),
			'+' => instructs.push(Instruction::Add),
			'-' => instructs.push(Instruction::Subtract),
			',' => instructs.push(Instruction::Read),
			'.' => instructs.push(Instruction::Write),

			// Loop section. When you reach the loop, move the contents of instructs into 'stack'. Then continue
			// with parsing, putting the new instructions into the now empty 'instructs'. When loop ends, pop the 'stack'
			// back onto instructs after removing the value of instructs into a new Instruction::Loop value.
			'[' => temp_stack.push(mem::take(&mut instructs)),
			']' => {
				if let Some(top) = temp_stack.pop() {
					let looping_insts = mem::replace(&mut instructs, top);
					instructs.push(Instruction::Loop(looping_insts));
				}
			},
			_ => ()
		};
	}
	
	instructs
}

fn execute(instrs: &Vec<Instruction>) {
	let min_grow_tape: u8 = 100;
	let mut tape_size: u16 = 30_000;
	let mut mem_tape: Vec<u8> = vec![0u8; tape_size as usize];
	let mut addr_pntr: usize = (tape_size/2) as usize;

	for inst in instrs {
		match inst {
			Instruction::IncrementPointer => addr_pntr += 1,
			Instruction::DecrementPointer => addr_pntr -= 1,
			Instruction::Add => mem_tape[addr_pntr] += 1u8,
			Instruction::Subtract => mem_tape[addr_pntr] -= 1u8,
			Instruction::Read => {
				let mut read_char = [0u8];
				if let Err(err) = std::io::stdin().read_exact(&mut read_char) {
					eprintln!("Failed to read input. Error:{}", err);
				}
				mem_tape[addr_pntr] = read_char[0];
			},
			Instruction::Write => print!("{}", mem_tape[addr_pntr] as char),
			Instruction::Loop(repeat) => {
				while mem_tape[addr_pntr] > 0 {
					execute(&repeat);
				}
			},			
		};
		
		if addr_pntr == 0 || addr_pntr == mem_tape.len() {
			tape_size += min_grow_tape as u16;
			addr_pntr += (min_grow_tape/2) as usize;

			mem_tape.resize(tape_size as usize, 0);
			mem_tape.rotate_right((tape_size/2) as usize);
		}
	}
}
