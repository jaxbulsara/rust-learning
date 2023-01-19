#[derive(Debug)]
enum UsState {
    Alabama,
    Wyoming,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn simple_match() {
    println!("The value of a penny: {}", value_in_cents(Coin::Penny));
    println!("The value of a nickel: {}", value_in_cents(Coin::Nickel));
    println!("The value of a dime: {}", value_in_cents(Coin::Dime));
    println!(
        "The value of a quarter: {}",
        value_in_cents(Coin::Quarter(UsState::Alabama))
    );
    value_in_cents(Coin::Quarter(UsState::Wyoming));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn option_match() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    dbg!(five);
    dbg!(six);
    dbg!(none);
}

fn main() {
    simple_match();
    option_match();
}
