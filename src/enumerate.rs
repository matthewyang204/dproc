pub fn sum(data: &[f64]) -> f64 {
	let mut sum: f64 = 0.0;
	for value in data {
		sum += value;
	}
	sum
}

pub fn count(data: &[f64]) -> f64 {
	data.len() as f64
}
