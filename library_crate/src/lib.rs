// Example library crate setup.
// No main.rs
// Just a lib.rs that defines the
// library's public API.
pub fn greet(name: &str) -> String { format!("Hello, {}!", name) }