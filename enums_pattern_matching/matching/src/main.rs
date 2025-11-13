fn main() {
    value_in_cents(Coin::Quarter(UsState::Texas));
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Washington,
    California,
    Virginia,
    Texas,
    NewYork,
    Columbia,
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
            println!("State quarter is from {state:?}!");
            25
        }
    }
}
