struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!(
            "Dropping CustomSmartPointer with data `{}",
            self.data
        );
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!(
        "CustomSmartPointers created.",
    );
    // c.drop(); // error: explicit destructor calls not allowed
    drop(c); // std::mem::drop is included in the prelude, so you can call drop() without the std::mem:: prefix.
    println!(
        "CustomSmartPointer dropped before the end of main.",
    );
}
