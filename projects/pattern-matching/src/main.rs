
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// TODO: fix warning: variants `Penny`, `Nickel`, and `Dime` are never constructed
fn main() {
    let my_coin_value = value_in_cents(Coin::Quarter);
    println!("Just picked a quarter! That's {} Cents!", my_coin_value);
}
