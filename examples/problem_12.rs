fn main() {
    let mut i = 0;
    loop {
        i += 1;
        let num = (1..=i).sum::<u64>();

        // you can check the square root of a number to get half of the factors
        // we only need the length of the factors, so we double the value
        // let divisibles = (1..=((num as f32).sqrt() as u64)).filter(|x| num % x == 0).count()*2;
        let divisibles =
            (1..=((num as f32).sqrt() as u64))
                .fold(0, |acc, x| if num % x == 0 { acc + 2 } else { acc });
        // println!("num: {} divisibles: {}", num, divisibles);
        if divisibles >= 500 {
            println!("{}", num);
            break;
        }
    }
}
