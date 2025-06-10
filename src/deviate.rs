// Load toplevel modules properly; this is a submodule
use crate::sort::sort;

pub fn range(data: &[f64]) -> f64 {
	let sortedData = sort(data);
	let max = sortedData[sortedData.len() - 1];
	let min = sortedData[0];
	let range = max - min;
	return range;
}
