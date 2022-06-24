fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let some_number = Some(5);
    let some_number = some_number.map(|x| x + 1);
}

// fn plus_one<T>(x: Option)
// where
//     T: isize,
// {
// }

enum IpAddrKind {
    V4(String),
    V6,
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
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

fn if_let() {
    let config_max = Some(3);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max)
    }
}
