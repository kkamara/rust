# add

## Usage

```bash
# This runs the code in adder/src/main.rs, which depends on the add_one crate.
cargo run -p adder
```

## Testing

```bash
# Run from workspace root or packages.
cargo test
# Or, from workspace root, specifying a package.
cargo test -p add_one
```