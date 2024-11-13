pub struct Counter {
    count: usize,
}

impl Counter {
    pub fn new() -> Self {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = usize; // The iterator will yield `usize` values.

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= 5 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter::new();

    // Using the iterator
    while let Some(value) = counter.next() {
        println!("{}", value);
    }
}
