pub fn abbreviate(phrase: &str) -> String {
    let split: Vec<_> = phrase.split(&[' ', '-', '_'][..]).collect();
    let mut acronym = String::new();
    
    for word in split {
        if word.is_empty() {
            continue;
        }

        let mut chars = word.chars();

        if let Some(first) = chars.next() {
            acronym.push(first);
        }

        if !word.chars().all(|c| c.is_uppercase()) {
            for c in chars {
                if c.is_uppercase() {
                    acronym.push(c);
                }
            }
        }
    }
    acronym.to_uppercase()
}
