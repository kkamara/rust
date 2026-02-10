#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!(
        "{}",
        value_in_cents(
            Coin::Quarter(
                UsState::Alaska
            )
        )
    );

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    // The following println!s will
    // error without the debugging
    // print {:?} e.g. {} will error.
    println!("five: {:?}", five);
    println!("none: {:?}", none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        // Note that we have to put
        // the catch-all arm last
        // because the patterns are
        // evaluated in order.
        // Also, we don't have to use
        // the other variable, like
        // _ => reroll(),
        // We can also ensure that,
        // explicitly, nothing happens
        // like
        // _ => (),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!(
                "State quarter from {:?}!",
                state
            );
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(1 + i),
    }
}

// The arms’ patterns must cover all
// possibilities.
// The following code will error.
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//     }
// }

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}