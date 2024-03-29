fn main() {
    let s = String::from("Hello world");
    let _hello = &s[..5];
    let _world = &s[6..];
    let _whole = &s[..];

    let word = first_word(&s[..]);
    println!("{}", word);
    let word1 = second_word(&s[..]);
    println!("{}", word1);
    // s.clear();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut l:[usize;2] = [0;2];
    let mut idx = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            l[idx] = i;
            idx += 1;
            if idx >= l.len() {
                return &s[l[0]..l[1]].trim();
            }
            continue;
        }
    }
    l[idx] = s.len();
    &s[l[0]..l[1]].trim()
}