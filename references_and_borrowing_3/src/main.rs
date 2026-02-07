fn main() {
    // Mutable references have one big
    // restriction: If you have a
    // mutable reference to a value,
    // you can have no other references
    // to that value.

    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // Won't compile because of this.

    println!("{r1}, {r2}");
}
