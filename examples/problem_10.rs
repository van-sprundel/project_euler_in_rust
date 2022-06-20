fn main() {
    let size = 2_000_000;
    let mut primes = vec![];

    for x in 1..=size {
        if is_prime(x) {
            primes.push(x);
        }
    }

    let mut res = 0;
    for x in primes {
        res += x as u64;
    }
    println!("{}", res);
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    let limit = (n as f64).sqrt() as u32;
    for a in 2..=limit {
        if n % a == 0 {
            return false;
        }
    }
    true
}
