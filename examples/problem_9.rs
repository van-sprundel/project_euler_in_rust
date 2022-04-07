fn main() {
    for a in 0..1000 {
        for b in 0..1000 {
            let c = unsafe { ((b * b + a * a) as f32).sqrt() };
            if (a as f32+ b as f32+ c) == 1000. {
                println!("found {}", a*b*c as i32);
            }
        }
    }
}