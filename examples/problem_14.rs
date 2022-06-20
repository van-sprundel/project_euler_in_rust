fn main() {
    let mut longest_count = 0;
    let mut longest_num = 0;
    for starting in 1..1_000_000 {
        let mut res: i64 = starting;
        let mut i = 0;
        loop {
            if res % 2 == 0 {
                res /= 2;
            } else {
                res *= 3;
                res += 1;
            }
            if res == 1 {
                if i > longest_count {
                    longest_count = i;
                    longest_num = starting;
                }
                break;
            }
            i += 1;
        }
    }
    println!("{} with size {}", longest_num, longest_count)
}
