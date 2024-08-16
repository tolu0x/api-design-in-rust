use std::{thread, time::Duration};

/// ANSI Escape Code
const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct Progress<Iter> {
    iter: Iter,
    i: usize,
}

trait ProgressIteratorExt {
    fn progress(self) -> Progress<Self>
    where
        Self: Sized;
}

impl<Iter> ProgressIteratorExt for Iter {
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}

impl<Iter> Progress<Iter> {
    fn new(iter: Iter) -> Self {
        Progress { iter, i: 0 }
    }
}

impl<Iter> Iterator for Progress<Iter>
where
    Iter: Iterator,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{}{}", CLEAR, "*".repeat(self.i));
        self.i += 1;
        self.iter.next()
    }
}

fn main() {
    let v = vec![1, 2, 3];
    for n in v.iter().progress() {
        expensive_calc(n)
    }
}

fn expensive_calc(_n: &i32) {
    thread::sleep(Duration::from_secs(1));
}
