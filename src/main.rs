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

// Load num-integer crate
mod num_integer;

fn userError() {
	eprintln!("ERROR: Invalid usage");
	exit(1);
}

fn version() {
	println!("dproc, version 1.0.1");
	println!("(C) 2013-2014 The Rust Project Developers");
	println!("(C) 2025 Matthew Yang");
	return;
}

fn help() {
	version();
	
	println!();
	println!("Usage: dproc [SUBCMD1] [SUBCMD2] {{yourdata}}");
	println!("");
	println!("SUBCMD1 options:");
	println!("  round        functions for rounding, getting averages, etc");
	println!("  deviate      functions for getting deviations and values related to deviations, such as the variance");
	println!("  organize     functions for organizing the dataset");
	println!("  enumerate    functions for enumerating (getting data about the dataset), such as getting their sum or counting how many values");
	println!("  math         miscellaneous functions for arbitrary things about integers like gcd and lcm");
	println!("");
	println!("SUBCMD2 options:");
	println!("");
	println!("  When SUBCMD1 is round:");
	println!("    mean           get the mean of the dataset");
	println!("    median         get the median of the dataset");
	println!("    mode           get the mode of the dataset");
	println!("    decimal        round to a decimal place, with the value to be rounded supplied as the first value, and the number of decimal places maintained specified as the second");
	println!("    integer        round the number to the nearest integer");
	println!("");
	println!("  When SUBCMD1 is deviate:");
	println!("    range          get the range of the data");
	println!("    variance       get the variance of the data; the program will ask you if the data given is a sample or population");
	println!("    standard       get the standard deviation of the data; the program will ask you if the data given is a sample or population");
	println!("    meanAbsolute   get the mean absolute deviation");
	println!("    medianAbsolute get the median absolute deviation");
	println!("");
	println!("  When SUBCMD1 is organize:");
	println!("    sort           sort the data from smallest to largest");
	println!("");
	println!("  When SUBCMD1 is enumerate:");
	println!("    sum            get the sum of the data");
	println!("    count          get the number of values in the data");
	println!("");
	println!("  When SUBCMD1 is math:");
	println!("    lcm            get the LCM of a dataset");
	println!("    gcd, gcf       get the GCD/GCF of a dataset");
	println!("Place your data, values separated by spaces, in the place of {{yourdata}}.");
}

fn getArgs() -> Vec<String> {
	let args: Vec<String> = env::args().collect();
	return args;
}

fn main() {
	let args = getArgs();
	
	if args.len() < 2 {
                help();
                userError();
        }

	if args[1] == "--version" || args[1] == "-v" || args[1] == "version" || args[1] == "v" {
		version();
		exit(0);
	} else if args[1] == "--help" || args[1] == "-h" || args[1] == "help" || args[1] == "h" {
		help();
		exit(0);
	}
	
	if args.len() < 4 {
		help();
		userError();
        }

	let data: Vec<f64> = args[3..]
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
	} else if args[1] == "math"{
		let data_integer: Vec<u32> = args[3..]
			.iter()
			.map(|x| x.parse::<u32>())
			.collect::<Result<Vec<_>, _>>()
			.unwrap_or_else(|_| {
				eprintln!("ERROR: This program can only calculate math properties of integers, exiting...");
				exit(1);
			});
		if args[2] == "lcm" {
			let result = data_integer.iter().cloned().reduce(|a, b| num_integer::lcm(a, b)).unwrap();
			println!("{}", result);
		} else if args[2] == "gcd" || args[2] == "gcf" {
			let result = data_integer.iter().cloned().reduce(|a, b| num_integer::gcd(a, b)).unwrap();
			println!("{}", result);
		} else if args[2] == "prime" {
			println!("Unimplemented STUB");
			exit(1);
			for value in data_integer {
				if num_integer::is_prime(value as i64) {
					print!("True ");
				} else {
					print!("False ");
				}
			}
			println!();
		}
	} else {
		help();
		userError();
	}
}
