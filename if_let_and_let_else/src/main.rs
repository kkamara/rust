fn main() {
    let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }
    // The following does the same thing.
    if let Some(max) = config_max { // It can't be `config_max = Some(max)` .
        println!("The maximum is configured to be {}", max);
    }

    // A let...else example from within
    // a function or method.
    // let Coin::Quarter(state) = coin else {
    //     return None;
    // };
}
