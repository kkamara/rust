fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // The method also works on a literal directly:
    let s = "initial contents".to_string();
    // The following is equivalent to
    // the "".to_string() method:
    let s = String::from("initial contents");

    // Strings are UTF-8 encoded:
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");
    // s.push('b'); // Push a single character.
    println!("The s string is: {s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note s1 has been moved here and can no longer be used.
                       // + signature is fn add(self, s: &str) -> String {}
                       // so we pass &s2 instead of s2.
    println!("The s3 string is: {s3}");
    // println!("{s1}"); // Line 42 statement actually takes
                         // ownership of s1, appends a copy
                         // of the contents of s2, and then
                         // returns ownership of the result.

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s1}"); // main retains ownership of s1
                      // when using format! macro
                      // because format! macro uses
                      // references.

    let s1 = String::from("hi");
    // let h = s1[0]; // Strings cannot be indexed like this.

    // A String is a wrapper over a Vec<u8>
    let hello = String::from("Hola"); // Len is 4 and the vector
                                      // storing the string "Hola"
                                      // is 4 bytes long.
    // Each of these letters takes 1 byte
    // when encoded in UTF-8.
    
    let hello = String::from("Здравствуйте");

    let hello = "Здравствуйте";
    // let answer = &hello[0];

    // If &"hi"[0] were valid code
    // that returned the byte value,
    // it would return 104, not h.

    // Bytes, Scalar Values, and Grapheme Clusters:
    //  https://doc.rust-lang.org/book/ch08-02-strings.html#bytes-scalar-values-and-grapheme-clusters

    let hello = "Здравствуйте";

    let s = &hello[0..4]; // &str

    println!("The s string is: {s}");

    // Here, s will be a &str that contains the
    // first 4 bytes of the string. Earlier, we
    // mentioned that each of these characters
    // was 2 bytes, which means s will be Зд.
    //  https://doc.rust-lang.org/book/ch08-02-strings.html#slicing-strings

    // If we were to try to slice only part of
    // a character’s bytes with something like
    // &hello[0..1], Rust would panic at runtime
    // in the same way as if an invalid index
    // were accessed in a vector.

    // You should use caution when creating
    // string slices with ranges, because doing
    // so can crash your program.

    // Iterating Over Strings.
    //  https://doc.rust-lang.org/book/ch08-02-strings.html#iterating-over-strings
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }

    // But be sure to remember that valid
    // Unicode scalar values may be made
    // up of more than 1 byte.
}