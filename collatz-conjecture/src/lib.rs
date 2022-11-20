pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut counter = 0;
    let mut n = Some(n);
    while n.unwrap() > 1 {
        counter += 1;
        match n.unwrap() % 2 {
            0 => n = Some(n.unwrap() / 2),
            _ => {
                n = n.unwrap().checked_mul(3);
                n?;
                n = n.unwrap().checked_add(1);
                n?;
            }
        }
    }
    Some(counter)
}
