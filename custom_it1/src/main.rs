struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

fn main() {
    let counter = Counter::new();
    for val in counter {
        println!("{}", val);
    }

    let counter2 = Counter::new();
    counter2.for_each(|x| {
        println!("Count: {}", x);
    });

    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~>");

    Counter::new().zip(Counter::new()).for_each(|x| {
        println!("({}, {})", x.0, x.1);
    });
}
