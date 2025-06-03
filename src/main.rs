// Regular imports
use std::env;
use std::process::exit;

// Load modules
mod enumerate;
mod round;
mod sort;

// Use functions from modules
use round::mean;
use sort::sort;

fn userError() {
	eprintln!("ERROR: Invalid usage");
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
	if args.len() < 4 {
		help();
		userError();
	}
	let data: Vec<f64> = args[2..].to_vec()
		.iter()
		.filter_map(|x| x.parse::<f64>().ok())
		.collect();
	
	if args[1] == "round" {
		if args[2] == "mean" {
			let result = mean(&data);
			println!("{}", result)
			println!("{}", result);
		}
	} else if args[1] == "organize" {
		if args[2] == "sort" {
			let result = sort(&data);
			for value in result {
				print!("{} ", value);
			}
			println!();
		}
	}
}
