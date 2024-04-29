use chrono::prelude::*;

fn nth(num: u32) -> String {
    format!("{}{}", num, match (num % 10, num % 100) {
        (1, 11) | (2, 12) | (3, 13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th",
    })
}

fn main() {
    let current_datetime = Local::now();
    let time = current_datetime.format("%H:%M").to_string();
    let daytime =  match current_datetime.hour() {
        0..=5 => "night",
        6..=11 => "morning",
        12..=17 => "afternoon",
        18..=23 => "evening",
        _ => "error",
    };

    let month = match current_datetime.month()
    {
        1 => "january",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "error",
    };


    println!("Hello it is {} in the {} on {} {}, if you want to know.", time, daytime, month, nth(current_datetime.day()));
}
