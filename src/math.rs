pub fn factorial(data: &[f64]) -> Vec<f64> {
    if data.is_empty() {
        eprintln!("WARNING: Your data is empty, returning an empty vector.");
        return vec![];
    }

    data.iter()
        .map(|&n| {
            if n < 0.0 || n.fract() != 0.0 {
                eprintln!(
                    "WARNING: Factorial is only defined for non-negative integers. Skipping {}.",
                    n
                );
                0.0
            } else {
                (1..=n as u64).map(|x| x as f64).product()
            }
        })
        .collect()
}
