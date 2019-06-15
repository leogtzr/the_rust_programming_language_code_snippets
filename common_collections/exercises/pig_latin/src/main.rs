fn car_cdr(s: &str) -> (&str, &str) {
    match s.chars().next() {
        Some(c) => s.split_at(c.len_utf8()),
        None => s.split_at(0),
    }
}

fn str_to_pig_latin(s: &str) -> String {
    let mut result = String::new();
    let char_crd = car_cdr(s);
    let first_char_str = String::from(char_crd.0);
    let first_char = first_char_str.to_ascii_uppercase().chars().next().unwrap();
    match first_char {
            'B' | 'C' | 'D' | 'F' | 'G' | 'H' | 'J' | 'K' | 'L' | 'M' | 'N' | 
            'P' | 'Q' | 'R' | 'S' | 'T' | 'V' | 'X' | 'Z' | 'W' | 'Y' => {
            result.push_str(char_crd.1);
            result.push('-');
            result.push(first_char);
            result.push_str("ay");
        }
        'A' | 'E' | 'I' | 'O' | 'U'  => {
            result.push_str(s);
            result.push_str("-hay");
        }
        _ => {}
    }
    result
}

fn main() {
    let text = "Hola mundo! first apple";
    for s in text.split_whitespace() {
        println!("[{}]", str_to_pig_latin(s));
    }

}
