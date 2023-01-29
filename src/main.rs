use std::env;

/*
const MIN_CELLS: u32 = 30_000; 
const MIN_ADD_CELLS: u16 = 500;
*/
//let mem: Vec<u8> = vec![0u8; MIN_CELLS as usize];

fn main() {
	// Get the CLI aruments the exe was called with.
	let args: Vec<String> = env::args().collect();

	if let Some(filepath) = args.get(1) {
		// Do stuff with filepath
		dbg!(&filepath);
	} else {
		eprintln!("Please provide a file to parse!");
	}
}
