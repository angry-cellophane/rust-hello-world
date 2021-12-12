fn main() {
    let s = String::from("Hello salut hola");
    let word = find_word(&s, 3);

    println!("{}", word);
}

fn find_word(s: &str, word_number: usize) -> &str {
    let bytes = s.as_bytes();
    let mut wn = 0;

    let mut start = 0;
    let mut start_defined = false;
    let mut end = 0;

    while end < bytes.len() {
        if !start_defined && bytes[end] != b' ' {
            start = end;
            start_defined = true;
        }
        if bytes[end] == b' ' {
            if wn < word_number {
                wn += 1;
                start_defined = false;
            } else {
                break;
            }
        }
        end += 1;
    }

    return &s[start..end];
}
