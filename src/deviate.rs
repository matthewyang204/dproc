//Regular imports
use std::process::exit;

// Load toplevel modules properly; this is a submodule
use crate::sort::sort;
use crate::round::mean;
use crate::round::median;
use crate::freq::mode;
use crate::coreFuncs::input;
use crate::enumerate::sum;

pub fn range(data: &[f64]) -> f64 {
	let sortedData = sort(data);
	let max = sortedData[sortedData.len() - 1];
	let min = sortedData[0];
	let range = max - min;
	return range;
}

pub fn variance(data: &[f64]) -> f64 {
	let meanData = mean(data);
	let mut diff = vec![];
	let mut tempStore: f64 = 0.0;
	for value in data {
		tempStore = value - meanData;
		diff.push(tempStore);
	}
	let mut diffSq = vec![];
	for value in diff {
		tempStore = value.powi(2);
		diffSq.push(tempStore);
	}
	let mut varianceVal: f64 = 0.0;
	let response1 = input("Is this a S)ample or a P)opulation? ");
	if response1.to_lowercase() == "p" {
		let sum = sum(&diffSq);
		varianceVal = sum / diffSq.len() as f64;
	} else if response1.to_lowercase() == "s" {
		let sum = sum(&diffSq);
		varianceVal = sum / (diffSq.len() as f64 - 1.0);
	} else {
		eprintln!("ERROR: Response invalid, exiting...");
		exit(1);
	}
	return varianceVal;
}

pub fn sd(data: &[f64]) -> f64 {
	let varianceVal = variance(data);
	let standardDev = varianceVal.sqrt();
	return standardDev;
}

pub fn meanAD(data: &[f64]) -> f64 {
	let meanData = mean(data);
	let mut deviations = vec![];
	let mut tempStore: f64 = 0.0;
	for value in data {
		tempStore = value - meanData;
		tempStore = tempStore.abs();
		deviations.push(tempStore);
	}
	let MAD = sum(&deviations) / deviations.len() as f64;
	return MAD;
}

pub fn medianAD(data: &[f64]) -> f64 {
        let medianData = median(data);
        let mut deviations = vec![];
        let mut tempStore: f64 = 0.0;
        for value in data {
                tempStore = value - medianData;
                tempStore = tempStore.abs();
                deviations.push(tempStore);
        }
        let MAD = median(&deviations);
        return MAD;
}

pub fn iqr(data: &[f64]) -> f64 {
	if data.is_empty() {
		eprintln!("WARNING: Your data is empty, so your value is also empty.");
		return 0.0;
	}
	let sortedData = sort(data);
	let q1_index = (sortedData.len() as f64 * 0.25).ceil() as usize - 1;
	let q3_index = (sortedData.len() as f64 * 0.75).ceil() as usize - 1;
	sortedData[q3_index] - sortedData[q1_index]
}

pub fn skewness(data: &[f64]) -> &str {
    let n = data.len();
    if n < 2 {
        return "sym";
    }

    let n_f64 = n as f64;
    let mean_val = mean(data);
    let stddev = sd(data);

    if stddev == 0.0 {
        return "sym";
    }

    let m3: f64 = data.iter().map(|x| (x - mean_val).powi(3)).sum::<f64>() / n_f64;

    let skew = m3 / stddev.powi(3);

    if skew > 0.0 {
        "pos"
    } else if skew < 0.0 {
        "neg"
    } else {
        "sym"
    }
}