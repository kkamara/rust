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

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => 1819 <= year,
            UsState::Alaska => 1959 <= year,
            UsState::Arizona => 1912 <= year,
            UsState::Arkansas => 1836 <= year,
            UsState::California => 1850 <= year,
            UsState::Colorado => 1876 <= year,
            UsState::Connecticut => 1788 <= year,
            UsState::Delaware => 1787 <= year,
            UsState::Florida => 1845 <= year,
            UsState::Georgia => 1788 <= year,
            UsState::Hawaii => 1959 <= year,
            UsState::Idaho => 1890 <= year,
            UsState::Illinois => 1818 <= year,
            UsState::Indiana => 1816 <= year,
            UsState::Iowa => 1846 <= year,
            UsState::Kansas => 1861 <= year,
            UsState::Kentucky => 1792 <= year,
            UsState::Louisiana => 1812 <= year,
            UsState::Maine => 1820 <= year,
            UsState::Maryland => 1788 <= year,
            UsState::Massachusetts => 1788 <= year,
            UsState::Michigan => 1837 <= year,
            UsState::Minnesota => 1858 <= year,
            UsState::Mississippi => 1817 <= year,
            UsState::Missouri => 1821 <= year,
            UsState::Montana => 1889 <= year,
            UsState::Nebraska => 1867 <= year,
            UsState::Nevada => 1864 <= year,
            UsState::NewHampshire => 1788 <= year,
            UsState::NewJersey => 1787 <= year,
            UsState::NewMexico => 1912 <= year,
            UsState::NewYork => 1788 <= year,
            UsState::NorthCarolina => 1789 <= year,
            UsState::NorthDakota => 1889 <= year,
            UsState::Ohio => 1803 <= year,
            UsState::Oklahoma => 1907 <= year,
            UsState::Oregon => 1859 <= year,
            UsState::Pennsylvania => 1787 <= year,
            UsState::RhodeIsland => 1790 <= year,
            UsState::SouthCarolina => 1788 <= year,
            UsState::SouthDakota => 1889 <= year,
            UsState::Tennessee => 1796 <= year,
            UsState::Texas => 1845 <= year,
            UsState::Utah => 1896 <= year,
            UsState::Vermont => 1791 <= year,
            UsState::Virginia => 1788 <= year,
            UsState::Washington => 1889 <= year,
            UsState::WestVirginia => 1863 <= year,
            UsState::Wisconsin => 1848 <= year,
            UsState::Wyoming => 1890 <= year,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let c = Coin::Quarter(
        UsState::Alaska
    );
    println!(
        "{}",
        value_in_cents(&c)
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

    println!(
        "{}",
        Some(
            String::from("test")
        ).unwrap_or(String::from("default value"))
    );

    println!("{:?}", describe_state_quarter(&c));
}

fn value_in_cents(coin: &Coin) -> u8 {
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

fn describe_state_quarter(coin: &Coin) -> Option<String> {
    // let...else syntax here.
    // See project if_let_and_let_else for
    // more.
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}