// Load toplevel modules properly; this is a submodule
use crate::sort::sort;
use crate::round::mean;
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
