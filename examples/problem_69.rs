extern crate num;

fn main() {
    println!("{}", compute(1000000));
}

fn compute(limit: u64) -> u64 {
    let mut n = 1;
    let mut p = 1;
    while n * p <= limit {
        p += 1;
        if is_relatively_prime(p, n) {
            n *= p;
        }
    }
    n
}

fn is_relatively_prime(a: u64, b: u64) -> bool {
    let (mut n1, mut n2) = (a, b);
    while n1 != n2 {
        if n1 > n2 {
            n1 -= n2;
        } else {
            n2 -= n1;
        }
    }
    n1 == 1
}
