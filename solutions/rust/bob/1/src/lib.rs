pub fn reply(message: &str) -> &str {
    let msg = message.trim();

    if msg.is_empty() {
        return "Fine. Be that way!";
    }

    let is_question = msg.ends_with('?');
    let is_yelling =
        msg.chars().any(|c| c.is_alphabetic()) &&
        msg.chars().all(|c| !c.is_alphabetic() || c.is_uppercase());

    match (is_yelling, is_question) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Whoa, chill out!",
        (false, true) => "Sure.",
        _ => "Whatever.",
    }
}