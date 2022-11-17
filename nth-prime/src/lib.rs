pub fn nth(n: u32) -> u32 {
    let mut counter = 0;
    let mut guess = 2;
    while counter < n {
        guess += 1;
        if is_prime(guess) {
            counter += 1;
        }
    }
    guess
}

fn is_prime(x: u32) -> bool {
    for i in 2..x {
        if x % i == 0 {
            return false;
        }
    }
    true
}
