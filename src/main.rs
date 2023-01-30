use std::env;
use std::fs;
use std::process;

/*
const MIN_CELLS: u32 = 30_000; 
const MIN_ADD_CELLS: u16 = 500;
*/
//let mem: Vec<u8> = vec![0u8; MIN_CELLS as usize];

fn main() {
	// Get the CLI aruments the exe was called with.
	let args: Vec<String> = env::args().collect();
	
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

	println!("{file_contents}");
}
