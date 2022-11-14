pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut remainder = n;
    for i in 2.. {
        while remainder % i == 0 {
            factors.push(i);
            remainder /= i;
        }
        if remainder == 1 {
            return factors;
        }
    }
    factors
}
