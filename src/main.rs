use std::{thread, time::Duration};

/// ANSI Escape Code
const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct Unbounded;
struct Bounded {
    bound: usize,
    delims: (char, char),
}

struct Progress<Iter, Bound> {
    iter: Iter,
    i: usize,
    bound: Bound,
}

trait ProgressDisplay: Sized {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>);
}

impl ProgressDisplay for Unbounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}", CLEAR);
        println!("{}", "*".repeat(progress.i));
    }
}

impl ProgressDisplay for Bounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}", CLEAR);
        println!(
            "{}{}{}{} {}/{}",
            progress.bound.delims.0,
            "*".repeat(progress.i),
            " ".repeat(self.bound - progress.i),
            progress.bound.delims.1,
            progress.i,
            self.bound
        );
    }
}

trait ProgressIteratorExt {
    fn progress(self) -> Progress<Self, Unbounded>
    where
        Self: Sized;
}

impl<Iter> ProgressIteratorExt for Iter {
    fn progress(self) -> Progress<Self, Unbounded> {
        Progress::new(self)
    }
}

impl<Iter> Progress<Iter, Unbounded> {
    fn new(iter: Iter) -> Self {
        Progress {
            iter,
            i: 0,
            bound: Unbounded,
        }
    }
}

impl<Iter> Progress<Iter, Unbounded>
where
    Iter: ExactSizeIterator,
{
    fn with_bounds(self) -> Progress<Iter, Bounded> {
        let bound = Bounded {
            bound: self.iter.len(),
            delims: ('[', ']'),
        };
        Progress {
            iter: self.iter,
            i: self.i,
            bound,
        }
    }
}

impl<Iter> Progress<Iter, Bounded> {
    fn with_delim(self, delim: char) -> Progress<Iter, Bounded> {
        Progress {
            iter: self.iter,
            i: self.i,
            bound: Bounded {
                bound: self.bound.bound,
                delims: match delim {
                    '[' => ('[', ']'),
                    '{' => ('{', '}'),
                    '<' => ('<', '>'),
                    _ => ('[', ']'),
                },
            },
        }
    }
}

impl<Iter, Bound> Iterator for Progress<Iter, Bound>
where
    Iter: Iterator,
    Bound: ProgressDisplay,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.bound.display(&self);
        self.i += 1;
        self.iter.next()
    }
}

fn main() {
    let v = (0..77).collect::<Vec<i32>>();
    for n in v.iter().progress().with_bounds().with_delim('{') {
        expensive_calc(n)
    }
}

fn expensive_calc(_n: &i32) {
    thread::sleep(Duration::from_millis(200));
}
