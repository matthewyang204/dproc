// Regular imports
use std::env;
use std::process::exit;

fn userError() {
	println!("ERROR: Invalid usage");
	exit(1);
}

fn version() {
	println!("Unimplemented STUB");
}

fn help() {
	version();
	println!("Unimplemented STUB");
}

fn getArgs() -> Vec<String> {
	let args: Vec<String> = env::args().collect();
	return args;
}

fn main() {
	let args = getArgs();
	
}
