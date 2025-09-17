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

pub fn min(data: &[f64]) -> f64 {
	if data.is_empty() {
		eprintln!("WARNING: Your data is empty, so your value is also empty.");
		return 0.0;
	}
	let mut min = data[0];
	for &value in data.iter() {
		if value < min {
			min = value;
		}
	}
	min
}

pub fn max(data: &[f64]) -> f64 {
	if data.is_empty() {
		eprintln!("WARNING: Your data is empty, so your value is also empty.");
		return 0.0;
	}
	let mut max = data[0];
	for &value in data.iter() {
		if value > max {
			max = value;
		}
	}
	max
}