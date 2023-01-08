fn main() {
    borrowing();
    mutable_reference();
}

fn borrowing() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn mutable_reference() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("Value was changed: {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
