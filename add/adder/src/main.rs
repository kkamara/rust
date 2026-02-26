use rand;

/*
    If crates in the workspace specify incompatible
    versions of the same dependency, Cargo will
    resolve each of them but will still try to
    resolve as few versions as possible.
 */

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
    println!("{num} plus two is {}!", add_two::add_two(num));
}