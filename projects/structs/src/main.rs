struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    basic_structs();
    tuple_structs();
    unit_like_structs();
}

fn basic_structs() {
    let user1 = build_user(
        String::from("someone@somesite.com"),
        String::from("someuser123"),
    );

    print_user(&user1);

    let user2 = User {
        email: String::from("another@somesite.com"),
        ..user1
    };

    print_user(&user2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    println!(
        "User(email={}, username={}, active={}, sign_in_count={})",
        user.email, user.username, user.active, user.sign_in_count,
    )
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs() {
    let red = Color(255, 0, 0);
    let origin = Point(0, 1, 2);

    println!("Color({}, {}, {})", red.0, red.1, red.2);
    println!("Point({}, {}, {})", origin.0, origin.1, origin.2);
}

#[derive(PartialEq)]
struct AlwaysEqual;

fn unit_like_structs() {
    let subject = AlwaysEqual;
    let object = AlwaysEqual;
    let equal = subject == object;

    println!("Always equal: {equal}");
}
