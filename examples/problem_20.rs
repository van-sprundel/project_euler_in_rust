use num::{BigInt, One};

fn main() {
    let res = c(100);
    println!("{}", res);
}

fn c(n: u32) -> u32 {
    (0..n)
        .fold(BigInt::one(), |res, i| res * BigInt::from(n - i))
        .to_string()
        .chars()
        .fold(0, |res, x| {
            let x = x.to_digit(10).expect("Couldn't parse digit");
            res + x
        })
}
