use num_integer::Roots;

// Checks if a number is prime, and a collection of other functions to use it and based around it
pub fn generate_primes(limit: usize) -> Vec<usize> {
    let mut numbers: Vec<usize> = (2..=limit).collect();
    let mut primes: Vec<usize> = Vec::new();

    while let Some(current_prime) = numbers.first().cloned() {
        primes.push(current_prime);
        numbers.retain(|&num| num % current_prime != 0);
    }

    primes
}
pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    } else if n == 2 || n == 3 {
        return true;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    for a in 2..n.isqrt() {
        if n % a == 0 {
            return false;
        }
    }
    return true;
}