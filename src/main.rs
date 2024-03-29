fn main() {
    let mut s = String::from("Hello, world!");
    let word = first_word(&s);
    println!("{}", word);
    let (start, end) = second_word(&s);
    println!("{} {}", start, end);
    s.clear();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn second_word(s:&String) -> (usize, usize) {
    let bytes = s.as_bytes();
    let mut l:[usize;2] = [0;2];
    let mut idx = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            l[idx] = i;
            idx += 1;
            if idx >= l.len() {
                return (l[0], l[1]);
            }
            continue;
        }
    }
    l[idx] = s.len();
    (l[0], l[1])
}