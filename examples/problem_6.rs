fn main() {
    let mut sum = 0;
    let mut sum_square = 0;
    for x in 0..=100 {
        sum += x;
        sum_square += x * x;
    }
    sum *= sum;
    println!("{} - {} = {}", sum, sum_square, sum - sum_square);
}
