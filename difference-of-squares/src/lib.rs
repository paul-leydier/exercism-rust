pub fn square_of_sum(n: u32) -> u32 {
    (1u32..=n).sum::<u32>().pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1u32..=n).fold(0, |total, x| total + x.pow(2))
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
