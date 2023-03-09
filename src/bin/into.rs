use idioms::into::{foo, foo2};

pub fn main() {
    const CODE: &str = include_str!("../lib/into.rs");

    println!("\n{}", CODE);

    println!("\nfoo() takes explicit None and Some()");

    foo("bar", None, None, None);
    foo("bar", Some(42), None, None);
    foo("bar", Some(42), Some(1337), Some(-1));

    println!("\nfoo2() takes None or actual values");

    foo2("bar", None, None, None);
    foo2("bar", 42, None, None);
    foo2("bar", 42, 1337, -1);
}
