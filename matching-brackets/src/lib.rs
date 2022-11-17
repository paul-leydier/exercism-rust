pub fn brackets_are_balanced(string: &str) -> bool {
    let mut buffer = Vec::new();
    for char in string.chars() {
        if char == '[' || char == '(' || char == '{' {
            buffer.push(char);
        }
        if char == ']' || char == ')' || char == '}'  {
            if buffer.is_empty() {
                return false;
            }
            match (buffer.pop().expect("oups"), char) {
                ('{', '}') | ('[', ']') | ('(', ')') => (),
                _ => return false,
            }
        }
    }
    buffer.is_empty()
}
