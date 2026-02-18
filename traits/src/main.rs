use std::fmt::Formatter;

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait Display {
    fn fmt(&self, f: &mut Formatter) -> String;
}

pub trait Debug {
    fn fmt(&self, f: &mut Formatter) -> String;
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// Trait bound syntax
pub fn notify_2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_3(item1: &impl Summary, item2: &impl Summary) {
}
// Forcing params to have the same type
// in a trait bound function.
pub fn notify_4<T: Summary>(item1: &T, item2: &T) {
}
// Param implementing multiple traits.
pub fn notify_5(item: &(impl Summary + Display)) {
}
pub fn notify_6<T: Summary + Display>(item: &T) {
}
// Unclear trait bound
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    1
}
// Clearer trait bound with where clause
fn some_function_2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}
// Return a value of some type that
// implements a trait.
// You can only use impl Trait if
// you’re returning a single type.
//  https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}

fn main() {
    println!("Hello, world!");
}


