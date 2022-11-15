pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = vec![0];
    'is_multiple: for x in 0..limit {
        for factor in factors {
            if *factor > 0 && x % factor == 0 {
                multiples.push(x);
                continue 'is_multiple;
            }
        }
    }
    multiples.iter().sum()
}
