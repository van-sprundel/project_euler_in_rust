use num::bigint::ToBigInt;
use num::pow::Pow;
use num::BigInt;

fn main() {
    let num = BigInt::from(2).pow(1000_u32).to_string();
    let res = num
        .chars()
        .fold(0, |mut res, x| res + x.to_digit(10).unwrap());

    println!("{}", res);
}
