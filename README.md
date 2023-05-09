# Is This Idiomatic Rust?

## Intro

Trying out a few things in Rust based on some other tutorials.

- [Idiomatic Rust](https://fettblog.eu/slides/idiomatic-rust) - contains further links

## Directory Structure

UPDATE: Not idiomatic Rust! 😄

Not a good way to proceed. Have a better understanding of modules now, so will not use this.

### What I Tried

I kind of like the structure I've got here. Seems a little more 'obvious' after using other languages. You may not like it.
NOTE: Test files have to be sub-modules to allow testing of module private functions using Rust's access rules.
      Previously, a separate tests directory was used but this prevents private module access.

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
    │   └── lib
    │       ├── counters.rs
    │       ├── file_parse
    │       │   └── file_parse_tests.rs
    │       ├── file_parse.rs
    │       └── into.rs
    └── target ...

### Details

- Cargo.toml

    ```toml
    # Our lib - 'idioms' with all modules in the 'lib' directory
    [lib]
    name = "idioms"
    path = "src/idioms_lib.rs"

    # Bins for running example for each added module
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

- lib/file_parse.rs
    ```rust
    // NOTE: This must be a sub-module of file_parse to enable private function testing!
    #[cfg(test)]
    #[path = "lib/file_parse/file_parse_tests.rs"]
    mod file_parse_tests;
    ```
