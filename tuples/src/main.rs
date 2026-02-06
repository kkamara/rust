fn main() {
    let mut x: (i32, f64, u8) = (500, 6.4, 1);

    x.1 = 200f64;

    println!("{}", x.1);
}
