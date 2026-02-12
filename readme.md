# rust

This repository follows the Rust Foundation's book The Rust Programming Language at https://doc.rust-lang.org/book .

## Notes

```
Struct values move from struct 1 to 
struct 2. Struct values not moved stay
where they were.
https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-with-struct-update-syntax

Tuple structs' values are accessed like
`example.[1]` .
https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-different-types-with-tuple-structs
Tuples are also accessed like this.
Also, you must specify the struct name
when destructuring a tuple struct.
`let Point(x, y, z) = origin;`
Destructuring a tuple would be like:
`let (x, y) = example;`

There are no null values or null pointers in safe Rust.

...[with the Option Enum] you have to 
convert an Option<T> to a T before you can 
perform T operations with it. Generally,
this helps catch one of the most common
issues with null: assuming that something
isn’t null when it actually is.
https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum
https://doc.rust-lang.org/std/option/enum.Option.html

`src/main.rs` and `src/lib.rs` are
called crate roots.
The reason for their name is that the
contents of either of these two files
form a module named crate at the root
of the crate’s module structure, known
as the module tree.
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
Notice that the entire module tree is
rooted under the implicit module named
crate.
https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#grouping-related-code-in-modules
```

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

## Create a library

```bash
cargo new --lib [package-name]
```