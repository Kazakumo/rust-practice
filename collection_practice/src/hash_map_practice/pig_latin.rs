pub fn pig_latin(string: &mut str) {
    let first_char = string.chars().nth(0);

    let aiueo = "aiueoAIUEO";

    let is_empty = match first_char {
        Some(_) => true,
        None => false,
    };
    if is_empty == false {
        return;
    };
}
