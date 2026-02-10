enum Coin {
    Penny,
    Nicket,
    Dime,
    Quarter,
}

fn main() {
    println!(
        "{}",
        value_in_cents(Coin::Penny)
    );
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nicket => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}