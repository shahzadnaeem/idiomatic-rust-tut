#[derive(Debug, Default)]
pub struct Counter {
    c: u32,
}

impl Counter {
    pub fn new(c: u32) -> Self {
        Counter { c }
    }

    // FIXME: Is this correct?
    pub fn iter(&mut self, to: u32) -> CounterIter {
        CounterIter::new(self, to)
    }
}

#[derive(Debug)]
pub struct CounterIter<'a> {
    counter: &'a mut Counter,
    to: u32,
}

impl<'a> CounterIter<'a> {
    pub fn new(counter: &'a mut Counter, to: u32) -> Self {
        CounterIter { counter, to }
    }
}

impl<'it> Iterator for CounterIter<'it> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut ctr = &mut self.counter;
        if ctr.c < self.to {
            let c = ctr.c;

            ctr.c += 1;

            Some(c)
        } else {
            None
        }
    }
}
