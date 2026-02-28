fn main() {
    #[derive(Debug)]
    struct Foo {
        bar: String,
    }
    let f = Foo {
        bar: "Hello".to_string(),
    };
    test(&f.bar);
    println!("{f:#?}");

    let s = "Hello".split("");
    let c: Vec<&str> = s.collect();
    println!("{c:?}");

    test_2(&["some", "text"]);
}

fn test(foo: &str) {
    println!("{foo:?}");
}

fn test_2(foo: &[&str]) {
    println!("{foo:?}");
}