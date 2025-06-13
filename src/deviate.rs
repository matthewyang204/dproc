// Load toplevel modules properly; this is a submodule
use crate::sort::sort;
use crate::round::mean;

pub fn range(data: &[f64]) -> f64 {
	let sortedData = sort(data);
	let max = sortedData[sortedData.len() - 1];
	let min = sortedData[0];
	let range = max - min;
	return range;
}

pub fn variance(data: &[f64]) -> f64 {
	let mean = mean(data);
	println!("Unimplemented STUB");
	println!("Returning 1.0 to handle STUB");
	return 1.0;
}
