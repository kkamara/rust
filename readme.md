# rust

This repository follows the Rust Foundation's book The Rust Programming Language at https://doc.rust-lang.org/book .

## Running the app

```bash
cargo run
```

A binary will be produced to the `./target/debug` directory.

## Building the app (debugging version)

```bash
cargo build
```

A binary will be produced to the `./target/debug` directory.

## Checking code compiles

```bash
cargo check
```

No binary will be produced.

## Building the app (production version)

```bash
cargo build --release
```

A binary will be produced to the `./target/release` directory.

## Cargo init a new project

```bash
cargo new [app_name]
```

Creates `src/main.rs` and `Cargo.toml` files. If you're not already within a Git folder, a `.gitignore` file and an empty `.git/` directory will be created.

When within a Git folder and you want your new Cargo project to be version-controlled, run `cargo new --vcs git [app_name]` .

## Cargo init an empty project directory

```bash
cargo init
```

Creates `src/main.rs` and `Cargo.toml` files. If you're not already within a Git folder, a `.gitignore` file and an empty `.git/` directory will be created.

When within a Git folder and you want your new Cargo project to be version-controlled, run `cargo init --vcs git` .

## Notes

```
Struct values move from struct 1 to 
struct 2. Struct values not moved stay
where they were.
https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-with-struct-update-syntax

Tuple structs' values are accessed like
`example.[1]` .
https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-different-types-with-tuple-structs
For example:
`let Point(x, y, z) = origin;`
Destructuring a tuple would be like:
`let (x, y) = example`
```