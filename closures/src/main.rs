fn main() {
    let c = |x: usize, y: usize| -> usize { x + y };
    println!("The sum of 1 and 2 is {}", c(1, 2));
    let plus_one = |x| x;
    println!("{}", plus_one(1)); // Sets whichever type gets used first, in this case usize.
    // println!("{}", plus_one("test")); // expected integer, found `&str` - because the type of `x` is set to `usize` by the first use of `plus_one`.
}
