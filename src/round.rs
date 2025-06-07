// Load toplevel modules properly; this is a submodule
use crate::enumerate::sum;

pub fn mean(data: &[f64]) -> f64 {
	if data.is_empty() {
		eprintln!("WARNING: Your data is empty, so your value is also empty.");
		return 0.0;
	}
	let sum: f64 = sum(data);
	sum / data.len() as f64
}

pub fn median(data: &[f64]) -> f64 {
	if data.is_empty() {
		eprintln!("WARNING: Your data is empty, so your value is also empty.");
		return 0.0;
	}
	println!("Unimplemented STUB");
	println!("Returning 1 to catch STUB");
	return 1.0;
}
