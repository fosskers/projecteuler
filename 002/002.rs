extern crate num;

use num::integer::Integer;
use std::iter::AdditiveIterator;
use std::mem;

fn main() {
    println!("Funtional Solution: {}", solve(4_000_000));
    println!("Iterative Solution: {}", solve2(4_000_000));
}

// --- Functional solution --- //
struct Fibonacci {
    curr: int,
    next: int,
}

// Borrowed from Rust by Example.
impl Iterator<int> for Fibonacci {
    fn next(&mut self) -> Option<int> {
        let new_next = self.curr + self.next;
        let new_curr = mem::replace(&mut self.next, new_next);

        Some(mem::replace(&mut self.curr, new_curr))
    }
}

fn fibs() -> Fibonacci {
    Fibonacci { curr: 1, next: 2 }
}

// It makes me happy to know this is possible in a C-like.
fn solve(limit: int) -> int {
    fibs()
        .take_while(|&n| n < limit)
        .filter(|&n| n.is_even())
        .sum()
}

// --- Iterative solution --- //
fn solve2(limit: int) -> int {
    let mut count = 2;  // Sum of even fibs < limit (4_000_000)
    let mut fst   = 1;
    let mut snd   = 2;
    let mut curr;
    
    loop {
        curr = fst + snd;

        if curr > limit {
            break
        } else if curr.is_even() {
            count += curr;
        }

        fst = snd;
        snd = curr;
    }

    return count;
}
