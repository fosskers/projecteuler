// Brings `sum` into scope.
use std::iter::AdditiveIterator;

fn div35(n: int) -> bool {
    n % 3 == 0 || n % 5 == 0
}

fn main() {
    let count2: int = range(1i, 1000).filter(|&n| div35(n)).sum();

    println!("Solution: {}", count2);
}
