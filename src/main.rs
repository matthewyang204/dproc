// Regular imports
use std::env;
use std::process::exit;
use std::io::{self, BufRead};

// Load modules
mod enumerate;
mod round;
mod sort;
mod deviate;
mod coreFuncs;
mod primes;

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
#[allow(unused_imports)]
use primes::generate_primes;
use primes::is_prime;

// Load crates
use num_integer;

// Include C functions from libmysolvers
unsafe extern "C" {
    fn quadratic_single_neg_solver(a: f64, b: f64, c: f64) -> f64;
    fn quadratic_single_pos_solver(a: f64, b: f64, c: f64) -> f64;

	fn linear_solver(val: i64, a1: f64, b1: f64, c1: f64, a2: f64, b2: f64, c2: f64) -> f64;
}

fn userError() {
	eprintln!("ERROR: Invalid usage");
	exit(1);
}

fn version() {
	println!("dproc, version 1.1.1");
	println!("(C) 2013-2014 The Rust Project Developers");
	println!("(C) 2025 Matthew Yang");
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
	println!("  solve        functions for solving equations");
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
	println!("    prime-check    check whether each number in the dataset is prime");
	println!("");
	println!("  When SUBCMD1 is solve:");
	println!("    quadratic-single {{a}} {{b}} {{c}}   solve a quadratic equation with the quadratic formula, where a, b, and c are the coefficients of the quadratic equation ax^2 + bx + c = 0. Note that you may have to convert your equation; this does not accept != 0 on the other side of the equation.");
	println!("    linear-dual {{a1}} {{b1}} {{c1}} {{a2}} {{b2}} {{c2}}   solve a system of two linear equations with the substitution method, where a1, b1, c1 are the coefficients of the first linear equation a1x + b1y = c1 and a2, b2, c2 are the coefficients of the second linear equation a2x + b2y = c2. Note that this is standard form and you may have to convert your equations to it.");
	println!("");
	println!("Place your data, values separated by spaces, in the place of {{yourdata}}. Alternatively, you may put `stdin` or `-` in the place of {{yourdata}} to read from stdin.");
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
	
	let mut data: Vec<f64> = Vec::new();
	if args[3] == "stdin" || args[3] == "-" {
		let stdin = io::stdin();
		let mut lines: Vec<String> = Vec::new();

		for line in stdin.lock().lines() {
			match line {
				Ok(content) => lines.push(content),
				Err(e) => eprintln!("ERROR: Error reading line from stdin: {}", e),
			}
		}

		data = lines
			.iter()
			.flat_map(|line| line.split_whitespace())
			.map(|x| x.parse::<f64>())
			.collect::<Result<Vec<_>, _>>()
			.unwrap_or_else(|_| {
				eprintln!("ERROR: There can only be valid numbers values in the dataset, exiting...");
				exit(1);
			});
	} else {
		data = args[3..]
			.iter()
			.map(|x| x.parse::<f64>())
			.collect::<Result<Vec<_>, _>>()
			.unwrap_or_else(|_| {
				eprintln!("ERROR: There can only be valid numbers values in the dataset, exiting...");
				exit(1);
			});
	}
	
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
		let data_integer: Vec<u32> = data
			.iter()
			.map(|x| *x as u32)
			.collect::<Vec<_>>();
		if args[2] == "lcm" {
			let result = data_integer.iter().cloned().reduce(|a, b| num_integer::lcm(a, b)).unwrap();
			println!("{}", result);
		} else if args[2] == "gcd" || args[2] == "gcf" {
			let result = data_integer.iter().cloned().reduce(|a, b| num_integer::gcd(a, b)).unwrap();
			println!("{}", result);
		} else if args[2] == "prime-check" {
			for value in data_integer {
				if is_prime(value as u32) {
					result.push("True");
				} else {
					result.push("False");
				}
			}
			for value in result {
				print!("{} ", value);
			}
			println!();
		} else {
			help();
			userError();
		}
	} else if args[1] == "solve"{
		if args[2] == "quadratic-single" {
			if data.len() < 3 {
				eprintln!("ERROR: Invalid usage of the quadratic solver");
				eprintln!("Usage: dproc solve quadratic {{a}} {{b}} {{c}}");
				eprintln!("Where a, b, and c are the coefficients of the quadratic equation ax^2 + bx + c = 0");
				eprintln!("WARNING: You may have to convert your equation; this does not accept != 0 on the other side of the equation.");
				exit(1);
			}
			let a: f64 = data[0];
			let b: f64 = data[1];
			let c: f64 = data[2];
			let root1 = unsafe { quadratic_single_pos_solver(a, b, c) };
			let root2 = unsafe { quadratic_single_neg_solver(a, b, c) };
			print!("{} ", root1);
			println!("{}", root2);
		} else if args[2] == "linear-dual" {
			if data.len() < 4 {
				eprintln!("ERROR: Invalid usage of the linear solver");
				eprintln!("Usage: dproc solve linear-dual {{a1}} {{b1}} {{c1}} {{a2}} {{b2}} {{c2}}");
				eprintln!("Where a1, b1, c1 are the coefficients of the first linear equation a1x + b1y + c1 = 0 and a2, b2, c2 are the coefficients of the second linear equation a2x + b2y + c2 = 0");
				exit(1);
			}
			let a1: f64 = data[0];
			let b1: f64 = data[1];
			let c1: f64 = data[2];
			let a2: f64 = data[3];
			let b2: f64 = data[4];
			let c2: f64 = data[5];
			let resultX = unsafe { linear_solver(0, a1, b1, c1, a2, b2, c2) };
			let resultY = unsafe { linear_solver(1, a1, b1, c1, a2, b2, c2) };
			print!("{} ", resultX);
			println!("{}", resultY);
		} else {
			help();
			userError();
		}
	} else {
		help();
		userError();
	}
}
