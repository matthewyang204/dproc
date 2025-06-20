// Regular imports
use std::env;
use std::process::exit;

// Load modules
mod enumerate;
mod round;
mod sort;
mod deviate;
mod coreFuncs;

// Use functions from modules
use round::mean;
use round::median;
use round::mode;
use deviate::range;
use deviate::variance;
use deviate::sd;
use deviate::meanAD;
use deviate::medianAD;
use sort::sort;
use enumerate::sum;
use enumerate::count;

fn userError() {
	eprintln!("ERROR: Invalid usage");
	exit(1);
}

fn version() {
	println!("dproc, version 1.0.0");
	println!("(C) 2025 Matthew Yang");
	return;
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
	
	if args[1] == "--version" || args[1] == "-v" || args[1] == "version" || args[1] == "v" {
		version();
		exit(0);
	}
	
	if args.len() < 4 {
		help();
		userError();
        }
	let data: Vec<f64> = args[2..]
		.iter()
		.map(|x| x.parse::<f64>())
		.collect::<Result<Vec<_>, _>>()
		.unwrap_or_else(|_| {
			eprintln!("ERROR: There can only be valid numbers values in the dataset, exiting...");
			exit(1);
		});
	
	if args[1] == "round" {
		if args[2] == "mean" {
			let result = mean(&data);
			println!("{}", result);
		} else if args[2] == "median" {
			let result = median(&data);
			println!("{}", result);
		} else if args[2] == "mode" {
			let result = mode(&data);
			for value in result {
				print!("{}", value);
			}
			println!();
		} else if args[2] == "decimal" {
			// let precisionString: f64 = args[4].parse().expect("Invalid number");
			let precision: usize = args[4].parse().expect("Not a valid integer");
			let value: f64 = args[3].parse().expect("Not a valid floating point number");
			let result = format!("{:.1$}", value, precision);
			println!("{}", result);
		} else if args[2] == "integer" {
			let mut result = vec![];
			for value in data {
				let rounded = value.round();
				result.push(rounded);
			}
			for value in result {
				print!("{} ", value);
			}
			println!();
		} else {
                        help();
                        userError();
                }
	} else if args[1] == "deviate" {
		if args[2] == "range" {
			let result = range(&data);
			println!("{}", result);
		} else if args[2] == "variance" {
			let result = variance(&data);
			println!("{}", result);
		} else if args[2] == "standard" {
			let result = sd(&data);
			println!("{}", result);
		} else if args[2] == "meanAbsolute" {
			let result = meanAD(&data);
			println!("{}", result);
		} else if args[2] == "medianAbsolute" {
			let result = medianAD(&data);
			println!("{}", result);
		} else {
                        help();
                        userError();
                }
	} else if args[1] == "organize" {
		if args[2] == "sort" {
			let result = sort(&data);
			for value in result {
				print!("{} ", value);
			}
			println!();
		} else {
                        help();
                        userError();
                }
	} else if args[1] == "enumerate" {
		if args[2] == "sum" {
			let result = sum(&data);
			println!("{}", result);
		} else if args[2] == "count" {
			let result = count(&data);
			println!("{}", result);
		} else {
			help();
			userError();
		}
	} else {
		help();
		userError();
	}
}
