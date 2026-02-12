// Example library crate setup.
// No main.rs
// Just a lib.rs that defines the
// library's public API.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
// See project binary_crate.
// See project backyard.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
