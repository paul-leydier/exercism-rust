pub fn reply(message: &str) -> &str {
    let cleaned = message.trim().replace('\n', "");
    if cleaned.is_empty() {
        return "Fine. Be that way!";
    }
    let is_shout = cleaned.chars().any(|c| c.is_alphabetic()) && cleaned.to_uppercase() == cleaned;
    let is_question = cleaned.ends_with('?');
    if is_question {
        if is_shout {
            return "Calm down, I know what I'm doing!";
        }
        return "Sure.";
    }
    if is_shout {
        return "Whoa, chill out!";
    }
    "Whatever."
}
