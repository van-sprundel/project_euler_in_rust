use std::ops::{Add, Sub};
use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};
use chrono::format::Numeric::IsoWeek;

fn main() {
    let start = Utc.ymd(1901, 1, 1);
    let end = Utc.ymd(2000, 12, 31);
    let duration = end.signed_duration_since(start).num_days();

    println!("{}", (1..=duration).fold(0, |mut res, x| {
        let day = start.add(Duration::days(x));
        if day.weekday() == Weekday::Sun && day.day() == 1 {
            res + 1
        } else {
            res
        }
    }));
}
