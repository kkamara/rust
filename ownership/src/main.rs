fn main() {
    let s = String::from("hello");

    takes_ownership(s);
    // Won't compile because s's value
    // was moved into the function and
    // is no longer valid here.
    // println!("{s}");

    let x = 5;

    makes_copy(x);
    // Will compile, because i32
    // implements the Copy trait.
    println!("{x}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) -> () {
    println!("{some_integer}");
}