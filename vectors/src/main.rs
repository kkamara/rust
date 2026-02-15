fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];
    // let mut v = Vec::new();
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);
    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is not third element."),
    }

    // let does_not_exist = &v[100]; // Panics.
    let does_not_exist = v.get(100);

    // References for the following lines:
    //  https://doc.rust-lang.org/book/ch08-01-vectors.html#reading-elements-of-vectors
    let first = &v[0]; // Because we did this, we
                       // cannot do the following push.
    
    // v.push(6); // Panics: mutable borrow occurs here.

    println!("The first element is: {first}");
    // println!("The v vector is: {v:#?}");

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("The v vector is: {v:#?}");

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
