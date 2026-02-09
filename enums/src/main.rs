#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(
        String::from("hello")
    );
    m.call();
    println!("{:#?}", m);

    let some_number = Some(5);
    let some_char = Some('e');

    // let absent_number: Option<i32> = None; // mismatched types error.

    println!("{:#?}", some_number);

    // ...[with the Option Enum] you have to 
    // convert an Option<T> to a T before you can 
    // perform T operations with it. Generally,
    // this helps catch one of the most common
    // issues with null: assuming that something
    // isn’t null when it actually is.
    // https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum
    // https://doc.rust-lang.org/std/option/enum.Option.html
}
