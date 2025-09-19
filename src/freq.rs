// Load required modules
use std::collections::HashMap;

pub fn mode(data: &[f64]) -> Vec<String> {
	let stringData: Vec<String> = data.iter().map(|x| x.to_string()).collect();
	let mut occurrences = HashMap::new();
	let mut max_count = 0;

	for value in stringData {
		let count = occurrences.entry(value).or_insert(0);
		*count += 1;
		if *count > max_count {
        		max_count = *count;
		}
	}
	
	occurrences
	    .into_iter()
	    .filter_map(|(val, count)| if count == max_count { Some(val) } else { None })
	    .collect()
}