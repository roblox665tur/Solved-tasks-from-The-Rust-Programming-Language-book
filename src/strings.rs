fn into_pig_latin(word: &str) -> String {
    if word.is_empty() || !word.is_ascii() {
        println!("Word is empty or non-ascii");
        return word.to_string();
    }
    let mut chars = word.chars();
    let first = chars.next().unwrap().to_ascii_lowercase();
    let rest: String = chars.collect();
    if is_vowel(first) {
        format!("{}hay", word)
    } else {
        format!("{}{}ay", rest, first)
    }
}
fn is_vowel(c: char) -> bool {
    match c.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' | 'y' => true,
        _ => false,
    }
}
