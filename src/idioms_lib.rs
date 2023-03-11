#[path = "lib/counters.rs"]
pub mod counters;

#[path = "lib/into.rs"]
pub mod into;

#[path = "lib/file_parse.rs"]
pub mod file_parse;

// NOTE: This must be a sub-module of file_parse to enable private function testing!
// NOTE: This is now optional as the filename is detected as a tests file by Rust
#[cfg(test)]
#[path = "lib/file_parse/file_parse_tests.rs"]
mod file_parse_tests;
