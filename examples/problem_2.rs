fn main() {
    let mut res = 0;
    let mut first = 0;
    let mut second = 1;
    let mut even_sum = 0;

    for i in 0..1_000_000 {
        if res > 4_000_000 { break; }
        res = first + second;
        first = second;
        second = res;
        if res % 2 == 0 {
            even_sum += res;
        }
    }

    println!("{}", even_sum);
}