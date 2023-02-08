use std::env;
use std::fs;
use std::process;
use std::mem;

const MIN_CELLS: u32 = 30_000; 
const MIN_ADD_CELLS: u16 = 100;

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
	let mut mem: Vec<u8> = vec![0u8; MIN_CELLS as usize];
	let mut addr_pntr: usize = (MIN_CELLS/2) as usize;

	// TODO: Finish "Read"
	for inst in instrs {
		match inst {
			Instruction::IncrementPointer => addr_pntr += 1,
			Instruction::DecrementPointer => addr_pntr -= 1,
			Instruction::Add => mem[addr_pntr] += 1u8,
			Instruction::Subtract => mem[addr_pntr] -= 1u8,
			Instruction::Read => (),
			Instruction::Write => print!("{}", mem[addr_pntr] as char),
			Instruction::Loop(repeat) => {
				while mem[addr_pntr] > 0 {
					execute(&repeat);
				}
			},			
		};

		//TODO: if approaching the end of vec, add to it.
	}
}
