#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { // param could be written as self: &Self
        self.width * self.height
    }
    fn width(&self) -> bool {
        0 < self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle { // Each struct is allowed to have multiple impl blocks.
    fn square(size: u32) -> Self { // Rectangle::square(x)
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "The area of the rectangle is {} sequare pixels.",
    //     area(width1, height1)
    // );

    // let rect1 = (30, 50);
    //
    // println!(
    //     "The area of the rectangle is {} sequare pixels.",
    //     area(rect1)
    // );

    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };
    //
    // println!(
    //     "The area of the rectangle is {} sequare pixels.",
    //     area(&rect1)
    // );
    //
    // println!("rect1 is {rect1:?}"); // single-line formatting
    // println!("rect1 is {rect1:#?}"); // multi-line formatting

    // let scale = 2;
    // let rect1 = Rectangle {
    //     width: dbg!(30 * scale), // dbg! sends output to stderr console.
    //     height: 50,
    // };
    //
    // dbg!(&rect1);

    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     rect1.area()
    // );

    // if rect1.width() {
    //     println!(
    //         "The rectangle has a nonzero width; it is {}",
    //         rect1.width
    //     )
    // }

//     The following is the same.
//     p1.distance(&p2);
//     (&p1).distance(&p2);

    let rect2 = Rectangle {
        width: 10,
        height: 40
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(30);
    println!("{:#?}", sq);
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }