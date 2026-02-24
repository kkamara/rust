/*
Rust can’t know which of the two parameters we need
for the output, so we need to tell it explicitly.
Note that the help text suggests specifying the same
lifetime parameter for all the parameters and the
output type, which is incorrect! Because contents
is the parameter that contains all of our text and
we want to return the parts of that text that match,
we know contents is the only parameter that should
be connected to the return value using the lifetime
syntax.
    https://doc.rust-lang.org/stable/book/ch12-04-testing-the-librarys-functionality.html#writing-a-failing-test
*/
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}