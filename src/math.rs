use num_bigint::BigUint;
use num_traits::{One, FromPrimitive};

pub fn factorial(data: &[f64]) -> Vec<BigUint> {
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
                BigUint::one()
            } else {
                (1..=n as u64).map(BigUint::from).product()
            }
        })
        .collect()
}
