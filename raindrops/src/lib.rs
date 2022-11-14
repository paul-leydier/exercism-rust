pub fn raindrops(n: u32) -> String {
    let mut result = String::new();
    if is_factor(n, 3) {
        result.push_str("Pling");
    }
    if is_factor(n, 5) {
        result.push_str("Plang");
    }
    if is_factor(n, 7) {
        result.push_str("Plong");
    }
    if result.is_empty() {
        result = n.to_string();
    }
    result
}

fn is_factor(a: u32, b:u32) -> bool {
    a % b == 0
}
