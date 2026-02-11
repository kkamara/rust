#[derive(Debug)]
struct Foo {
    a: u8,
    b: u8,
    c: String,
}

fn main() {
    let s1 = Foo {
        a: 1,
        b: 2,
        c: String::from("test"),
    };
    // If we don't specify a new c string, we get
    // the compiler error:
    // borrow of partially moved value: `s1`
    // partial move occurs because `s1.c` has type
    // `String`, which does not implement the `Copy`
    // trait
    let mut s2 = Foo {
        c: String::from("test2"),
        ..s1
    };
    println!("s1: {:#?}", &s1);
    println!("s2: {:#?}", &s2);
    s2.c = String::from("test3");
    println!("s1: {:#?}", &s1);
    println!("s2: {:#?}", &s2);
    /* Output:
        s1: Foo {
            a: 1,
            b: 2,
            c: "test",
        }
        s2: Foo {
            a: 1,
            b: 2,
            c: "test2",
        }
        s1: Foo {
            a: 1,
            b: 2,
            c: "test",
        }
        s2: Foo {
            a: 1,
            b: 2,
            c: "test3",
        }
     */
}
