# minigrep

## Usage

```bash
# Minigrep is case sensitive by default.
# See below for running this app while ignoring the case.
cargo run -- search_string example-filename.txt
```

#### Working Examples

```bash
IGNORE_CASE=1 cargo run -- to poem.txt
IGNORE_CASE=0 cargo run -- to poem.txt
```