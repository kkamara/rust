use std::sync::mpsc;
use std::thread;
use std::time::Duration;
// mpsc stands for multiple producer, single consumer.
fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(
                Duration::from_secs(1)
            );
        }
    });
    
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(
                Duration::from_secs(1)
            );
        }
    });
    // .recv() will block the main thread
    // until a value is sent from the
    // spawned thread.
    // let received = rx.recv().unwrap();
    // There is also .try_recv() which
    // is non-blocking.
    // println!("Got: {received}");

    for received in rx {
        println!("Got: {received}");
    }
}
