pub fn build_proverb(list: &[&str]) -> String {
    let mut out = String::new();
    for i in 0..list.len() {
        if i == list.len() - 1 {
            out.push_str(format!("And all for the want of a {}.", list[0]).as_str());
        } else {
            out.push_str(
                format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]).as_str(),
            );
        }
    }
    out
}
