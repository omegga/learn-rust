fn main() {
    iterators();
    next_item();
    sum();
    map();
    creating_an_iterator();
}

fn iterators() {
    // An iterator is responsible for the logic of iterating over
    // each item and determining when the sequence has finished.
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // In Rust, iterators are lazy and have no effect until
    // you call methods that consume the iterator

    for val in v1_iter {
        println!("val = {}", val);
    }
}

fn next_item() {
    // All iterators implement a trait named "Iterator"
    // that requires implementors to define the "next" method,
    // which returns one item of the iterator wrapped in Some,
    // and when iteration is over, None.

    // We can call the "next" method directly on iterators
    let v1 = vec![1, 2, 3];
    // We need to make v1_iter mutable.
    // Calling .next() changes internal state of the iterator
    // to keep track of where it is in the sequence.
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // for loops take ownership of iterator and make it mutable behind
    // the scenes.

    // Values we get from next are immutable references to the values
    // in the vector.

    // Using into_iter() instead of iter(), we can create an iterator that
    // takes ownership of v1 and returns owned values.

    // If we want to iterate over mutable references we can call iter_mut()
}

fn sum() {
    let v1 = vec![1, 2, 3];
    let total: i32 = v1.iter().sum();
    assert_eq!(total, 6);
}

fn map() {
    // Other methods defined on the Iterator trait can produce other iterators
    let v1: Vec<i32> = vec![1, 2, 3];
    // map takes a closure to call on each item to produce a new iterator
    // collect consumes the iterator and produce a collection data type
    let v1: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v1);
}

fn creating_an_iterator() {
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32; // associated type
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
