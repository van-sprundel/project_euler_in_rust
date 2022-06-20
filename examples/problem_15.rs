fn main() {
    println!("{}", solve(20));
}

fn solve(n: usize) -> usize {
    (1..=n).fold(1, |mut ret, i| {
        ret *= n + i;
        ret / i
    })
}
