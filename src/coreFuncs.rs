use std::io::{self, Write};

fn input(prompt: &str) -> String {
	print!("{}", prompt);
	io::stdout().flush().unwrap();

	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();

	input.trim().to_string()
}
