pub fn sort(data: &[f64]) -> Vec<f64> {
	let mut toSort: Vec<f64>  = data.to_vec();
	toSort.sort_by(|a, b| a.partial_cmp(b).unwrap());
	toSort
}

pub fn keepUnique(data: &[f64]) -> Vec<f64> {
	let mut toSort: Vec<f64>  = data.to_vec();
	toSort.sort_by(|a, b| a.partial_cmp(b).unwrap());
	toSort.dedup();
	toSort
}