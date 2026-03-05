use std::sync::Mutex;
// https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html#the-api-of-mutext
fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
}
