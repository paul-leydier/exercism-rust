pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    let computed: u32 = digits.iter().map(|x| x.pow(digits.len() as u32)).sum();
    computed == num
}
