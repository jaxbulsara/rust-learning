fn main() {
    let s1 = "Hello world!";
    let first = first_word(&s1);
    let last = last_word(&s1);

    println!("The first word is {}", first);
    println!("The last word is {}", last);
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

fn last_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().rev().enumerate() {
        if item == b' ' {
            return &s[i..];
        }
    }

    &s[..]
}
