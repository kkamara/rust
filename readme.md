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
See restaurant project for application.

When to use self (The Instance)
Use lowercase self as the first parameter
of a method when you want to call that
method on an instance of the struct (e.g.,
my_struct.do_something()). There are three
primary forms: 
- &self (Immutable Borrow): Use this most
of the time. It allows the method to read
data from the struct without taking
ownership or changing anything.
- &mut self (Mutable Borrow): Use this
when the method needs to modify the
struct's fields.
- self (Taking Ownership): Use this when
the method needs to "consume" the struct,
such as transforming it into another type
or dropping it after use.
Google Gemini AI.

Monomorphization: the process of
converting generics into concrete
types at compile time, in Rust.

Common generics: Result<T, E> and
Option<T> .

In Rust, Option<T> is an enum used to represent
a value that may or may not be present. It has
two variants: Some(T) (contains a value) and
None (empty).

`pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>`
query is implicitly a lifetime of type 'b.
Google Gemini AI confirmed this, my explanation,
is "exactly" the case.
Flexibility: You allow the query string to
be dropped immediately after the function
finishes, while the returned Vec can still
be used as long as contents is still around.
Google Gemini AI.

`unimplemented!();`

Tests run in parallel by default. We can
ensure single threaded testing with
`cargo test -- --test-threads=1`
which is useful when tests share state.

Explaining ownership and borrowing rules
to my non-technical friend (1):
When you pass a variable to another part of
the code, a function, you can change
ownership of the variable to the function,
or you can make the function borrow the
variable.

Explaining ownership and borrowing rules
to my non-technical friend (2) (improved):
You change which part of the code OWNS a
variable. Or you let other parts of the
code BORROW a variable.

Helpful macros:
`unimplemented!();`
`todo!();`
`unreachable!();`

`Box<T>` copies values onto the heap.
See project my_box.
Also do this (above line) for implementing
the Deref trait (deref drop function): for
deciding what happens when the type is
cleaned up (dropped from memory).

For concurrency related projects, see:
- threads
- channels
- mutex
- mutex_2
- hello-async
- hello (practical and last project of the
  course)

I completed the Rust book in 30 days.
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
