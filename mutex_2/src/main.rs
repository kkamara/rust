use std::sync::{Mutex, Arc};
use std::thread;
// https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html#comparing-refcelltrct-and-mutextarct
fn main() {
    // Arc stands for atomic reference counting.
    // It is a thread-safe version of Rc<T>.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}