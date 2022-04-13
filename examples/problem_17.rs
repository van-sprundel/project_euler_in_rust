fn main() {
    println!("{}", (1..=1000).fold(0, |res, i| { res + num_to_word(i).len() }));
}

fn num_to_word(n: usize) -> String {
    if n == 0 { return "zero".to_string(); }
    let mut under_10 = match n % 10 {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => ""
    }.to_owned();
    if n < 10 { return under_10; }

    let mut under_100 = match (n % 100) / 10 {
        1 => match n % 10 {
            0 => "ten",
            1 => "eleven",
            2 => "twelve",
            3 => "thirteen",
            5 => "fifteen",
            8 => "eighteen",
            _ => {
                under_10.push_str("teen");
                &under_10
            }
        },
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => ""
    }.to_owned();
    if !(11..20).contains(&(n % 100)) {
        under_100.push_str(&*under_10);
    }
    if n < 100 {
        return under_100;
    }

    let under_1000 = (match (n % 1000) / 100 {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => ""
    }).to_owned() + "hundred";

    if n < 1000 {
        let mut temp = under_1000;
        if n % 100 != 0 { temp += "and" }
        temp.push_str(&*under_100);
        return temp;
    }
    "onethousand".to_string()
}