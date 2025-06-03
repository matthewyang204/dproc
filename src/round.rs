// Load modules required for functions here
mod enumerate;

// Use functions from these modules
use enumerate::sum;

fn mean(data: &[f64]) -> f64 {
	if data.is_empty() {
		eprintln!("WARNING: Your data is empty, so your value is also empty.");
		return 0.0;
	}
	let sum: f64 = sum(data);
	sum / data.len() as f64
}
