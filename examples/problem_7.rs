fn main() {
    let mut nth = 10_001;
    let mut i = 0;
    let mut count = 1;
    loop {
        i += 1;
        let max_p = (i as f64).sqrt().ceil() as u64;
        if i % 2 == 0 || i % 3 == 0 { continue; }
        if (5..=max_p).step_by(6).find(|p| i % p == 0 || i % (p + 2) == 0).is_some() { continue; }
        count += 1;

        if nth == count {
            break;
        }
    }
    println!("{}", i);
}