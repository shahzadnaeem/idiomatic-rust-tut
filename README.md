# Is This Idiomatic Rust?

## Intro

Trying out a few things in Rust based on some other tutorials.

- [Idiomatic Rust](https://fettblog.eu/slides/idiomatic-rust) - contains further links

## Directory Structure

I kind of like the structure I've got here. Seems a little more 'obvious' after using other languages. You may not like it.

    ├── Cargo.lock
    ├── Cargo.toml
    ├── README.md
    ├── src
    │   ├── bin
    │   │   ├── counters.rs
    │   │   ├── file_parse.rs
    │   │   └── into.rs
    │   ├── data
    │   │   └── data.txt
    │   ├── idioms_lib.rs
    │   ├── lib
    │   │   ├── counters.rs
    │   │   ├── file_parse.rs
    │   │   └── into.rs
    │   └── tests
    └── target...

### Details

- Cargo.toml

    ```toml
    # Our lib - 'idioms' with all packages in the 'lib' directory
    [lib]
    name = "idioms"
    path = "src/idioms_lib.rs"

    # Bins for running example for each added package
    [[bin]]
    name = "counters"
    ```

- idioms_lib.rs

    ```rust
    #[path = "lib/counters.rs"]
    pub mod counters;

    #[path = "lib/into.rs"]
    pub mod into;

    #[path = "lib/file_parse.rs"]
    pub mod file_parse;
    ```
