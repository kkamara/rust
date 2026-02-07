fn main() {
    // Mutable references have one big
    // restriction: If you have a
    // mutable reference to a value,
    // you can have no other references
    // to that value.

    let mut _s = String::from("hello");

    // let r1 = &mut _s;
    // let r2 = &mut _s; // Won't compile because of this.

    // println!("{r1}, {r2}");

    // The following is fine.

    let mut s = String::from("hello");

    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let _r2 = &mut s;
}
