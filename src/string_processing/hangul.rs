use rustkorean::check_korean;

pub fn is_include_hangul(characters: &str) -> bool {
    for c in characters.chars() {
        if check_korean(c) {
            return true;
        }
    }
    false
}
