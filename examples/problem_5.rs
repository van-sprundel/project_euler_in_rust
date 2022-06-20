fn main() {
    for i in 20..=u32::MAX {
        let mut correct = true;
        for set in 2..=20 {
            if (i as f64 / set as f64).fract() != 0.0 {
                correct = false;
                break;
            }
        }
        if correct {
            println!("{}", i);
            break;
        }
    }
}
