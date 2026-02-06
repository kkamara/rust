fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if 10 == counter {
            break counter * 2;
        }
    };
    // The result is 20
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if 9 == remaining {
                break;
            }
            if 2 == count {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    let mut number = 3;

    while 0 != number {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("LIFTOFF!!!");
}