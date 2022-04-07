
fn main() {
    let mut res = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            let product = x*y;
            let chars = product.to_string();
            let char_length = chars.len();
            let mut correct = true;
            for i in 0..(char_length/2) {
                let left =  chars.chars().nth(i).unwrap();
                let right =  chars.chars().nth(char_length-1-i).unwrap();
                if left != right {
                    correct = false;
                    break;
                }
            }
            if correct && product > res{
                res = product;
            }
        }
    }
    println!("res: {}",res);
}