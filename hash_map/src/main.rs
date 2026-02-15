use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name)
        .copied() // calling copied to get an
                  // Option<i32> rather than
                  // an Option<&i32>
        .unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this
    // point.
    // println!("{field_name}: {field_value}");

    // If we insert references to values into the
    // hash map, the values won’t be moved into the
    // hash map. The values that the references
    // point to must be valid for at least as long
    // as the hash map is valid.

    // Overwriting a Value.
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{scores:?}");

    // Adding a Key and Value Only If a Key Isn’t
    // Present.
    //  https://doc.rust-lang.org/book/ch08-03-hash-maps.html#adding-a-key-and-value-only-if-a-key-isnt-present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow"))
        .or_insert(50);
    scores.entry(String::from("Blue"))
        .or_insert(50);

    println!("{scores:?}"); // scores["Blue"] will be 10, not 50.

    // Updating a Value Based on the Old 
    let text = "hello  world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // The or_insert method returns a
        // mutable reference (&mut V) to the
        // value for the specified key.
        let count = map.entry(word)
            .or_insert(0);
        // Here, we store that mutable
        // reference in the count variable,
        // so in order to assign to that
        // value, we must first dereference
        // count using the asterisk (*).
        *count += 1;
    }

    println!("{map:?}");

    // Line 47, line 59:
    // The or_insert method on Entry is defined
    // to return a mutable reference to the
    // value for the corresponding Entry key if
    // that key exists, and if not, it inserts
    // the parameter as the new value for this
    // key and returns a mutable reference to
    // the new value.

    // Hashing Functions.
    //  https://doc.rust-lang.org/book/ch08-03-hash-maps.html#hashing-functions
}
