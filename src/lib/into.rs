use std::fmt::Debug;

pub fn foo(lorem: &str, ipsum: Option<i32>, dolor: Option<i32>, sit: Option<i32>) {
    println!("foo: {}, {:?}, {:?}, {:?}", lorem, ipsum, dolor, sit);
}

pub fn foo2<I, D, S>(lorem: &str, ipsum: I, dolor: D, sit: S)
where
    I: Into<Option<i32>> + Debug,
    D: Into<Option<i32>> + Debug,
    S: Into<Option<i32>> + Debug,
{
    println!("foo2: {}, {:?}, {:?}, {:?}", lorem, ipsum, dolor, sit);
}
