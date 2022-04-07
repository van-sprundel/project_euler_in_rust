fn main() {
    let num = 600851475143;
    let mut list = vec![];
    for x in 1..=num {
        if (x % 2 != 0 || x % 3 != 0)&& num % x == 0 {
            let max_p = (x as f64).sqrt().ceil() as u64;
            if (5..=max_p).step_by(6).find(|p| x % p == 0 || x % (p+2) == 0).is_some() {break;}
            list.push(x);
        }
    }
    println!("{:?}", list);
}