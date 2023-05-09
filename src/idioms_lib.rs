#[path = "lib/counters.rs"]
pub mod counters;

#[path = "lib/into.rs"]
pub mod into;

#[path = "lib/file_parse.rs"]
pub mod file_parse;

// NOTE: It is possible to defined tests for modules here, but it affects the module nesting
//       For example, defining 'file_parse_tests' here will no longer allow a simple 'use super::*;'
//       to access all 'file_parse' members. It also prevents any 'private' tests
//
//       See #[cfg(test)] at the end of 'lib/file_parse.rs'

#[path = "lib/structures.rs"]
pub mod structures;

#[path = "lib/base/lib.rs"]
pub mod base;
