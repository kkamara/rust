use std::fmt::Formatter;

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

fn main() {
    println!("Hello, world!");
}
