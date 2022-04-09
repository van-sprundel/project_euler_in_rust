extern crate core;

use std::ops::{Add, Div, Mul, Sub};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use num::{BigInt, ToBigInt};
use num::{Inv, One, pow, Pow, ToPrimitive, Zero};
use num::bigint::ToBigInt;
use num::real::Real;
use crate::pow::Pow;

fn main() {
    println!("{}", v2(BigInt::from(24))); // 3
    println!("{}", s(4)); // 980
    println!("{}", u(4)); // 7
    println!("{}", u(20)); // 24
    println!("answer: {}", u2(5)); // 241


    // println!("{}", u2(10_000)); // ?
}

pub const TWO: isize = 2;

fn v2(n: BigInt) -> isize {
    // O(1) for anything that wouldn't return anything other than 1 or the sqrt of itself;
    if &n % 2.to_bigint().unwrap() != 0.to_bigint().unwrap() || n < BigInt::zero() { return 1; }

    let mut res = 1;
    // y = 2^x
    // x = ln(y)/ln(2)
    // so max is only 0..x, diving bigger than n won't work
    let max = (((n.to_f32().unwrap()).ln() / 2_f32.ln()) as isize);
    // println!("max of {:02} is {:02}",n,max);

    // println!("num {}", &n);
    for i in 2..=max {
        // let div = &n.to_f64().unwrap().mul((i as f64).recip());
        // if div.fract() == 0.0 {
        //     let x = div.ln() / 2_f64.ln();
        //     if x as isize > max { continue; }
        //     // println!("round number {} {}", div, x);
        //     if x.fract() == 0.0 {
        //         println!("Found em!");
        //         return x as isize;
        //     }
        // }


        let i_end = max.sub(i);
        // println!("{:02} and {:02} out of {}", i_start, i_end, max);
        let div_start = BigInt::from(TWO.to_f32().unwrap().pow(i as f32) as isize);

        let div_end = BigInt::from(TWO.to_f32().unwrap().pow(i_end as f32) as isize);
        if &n.to_bigint().unwrap() % div_end == BigInt::zero() {
            return i_end; // we only need the highest one
        }
        if &n.to_bigint().unwrap() % div_start == BigInt::zero() {
            // println!("found {}",i);
            res = i; // update res
        }
    }
    res
}

// s(4) = 980
// s(20) = 11_184_809
fn s(n: isize) -> BigInt {
    (1..=n).fold(0.to_bigint().unwrap(), |res, k| {
        res + BigInt::from(-2).pow(k as u32) * BigInt::from(binom(2 * k, k))
    })
}

// binom function from https://programming-idioms.org/idiom/67/binomial-coefficient-n-choose-k/747/rust
fn binom(n: isize, k: isize) -> BigInt {
    let mut res = BigInt::one();
    for i in 0..k {
        res = (res * (n - i)) /
            (i + 1);
    }
    res
}


fn u(n: isize) -> isize {
    v2(3.to_bigint().unwrap() * s(n) + 4.to_bigint().unwrap())
}

fn u2(n: isize) -> isize {
    (1..=n).fold(0, |mut res, n| {
        res += u(n.pow(3_u32));
        res
    })
}


fn find_v2(n: isize) -> isize {
    // 2^n..
    let mut i = 2_isize.pow(n as u32);
    loop {
        i += 4; // because of 2^n, only every 4 step is going to give a result other than 1
        let res = v2(i.to_bigint().unwrap());
        // println!("{:02} : {}", i, res);

        if res == (n) {
            return i;
        }
    }
}