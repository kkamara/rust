struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    /*
        Because Rust compiles generic code into
        code that specifies the type in each
        instance, we pay no runtime cost for
        using generics. When the code runs, it
        performs just as it would if we had
        duplicated each definition by hand. The
        process of monomorphization makes
        Rust’s generics extremely efficient at
        runtime.
     */
}