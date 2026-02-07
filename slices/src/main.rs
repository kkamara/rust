fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    // These are equal.
    let _slice = &s[0..2]; // _slice for the compiler to not
                           // give the unused variable warning.
    let slice = &s[..2];
    println!("{}", slice);

    // These are also equal.
    let len = s.len();

    let _slice = &s[3..len];
    let _slice = &s[3..];

    // These are equal too.
    let _slice = &s[0..len];
    let _slice = &s[..];

    // Some other stuff.
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3]; // Slice of an array.

    assert_eq!(slice, &[2, 3]);
}
