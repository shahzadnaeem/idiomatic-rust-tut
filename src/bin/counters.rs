use idioms::counters::Counter;

pub fn main() {
    let mut counter = Counter::new(3);

    let nums = counter.iter(10).collect::<Vec<_>>();

    println!("nums: {:?}, counter: {:?}", nums, counter);
}
