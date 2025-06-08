// Load toplevel modules properly; this is a submodule
use crate::enumerate::sum;
use crate::sort::sort;

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
	let sortedData = sort(data);
	if sortedData.len() % 2 == 1 {
		return sortedData[data.len() / 2];
	} else {
		let mid1 = sortedData[data.len() / 2];
		let mid2 = sortedData[(data.len() / 2) - 1];
		return (mid1 + mid2) / 2.0;
	}
	println!("Unimplemented STUB");
	println!("Returning 1 to catch STUB");
	return 1.0;
}
