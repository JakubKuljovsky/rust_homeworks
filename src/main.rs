
use english_numbers::convert_all_fmt;
use std::env;
fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut lowercase : bool = false;
    let mut uppercase : bool = false;
    let mut no_space : bool = false;
    let mut slugify: bool = false;
    let mut reverse : bool = false;
    let mut convert_numbers_to_words : bool = false;

    for str in args.clone() {
        match str.as_ref(){
            "reverse" => reverse = true,
            "convert-numbers-to-words" => convert_numbers_to_words = true,
            "lowercase" => lowercase = true,
            "uppercase" => uppercase = true,
            "slugify" => slugify = true,
            "no-spaces" => no_space = true,
            _ => ()
        }
    }

    if reverse  {
        args =  args.clone().into_iter().rev().collect();
    }

    if convert_numbers_to_words {
            for i in 1..args.len() {
                match args[i].parse::<i64>() {
                    Ok(n) => args[i] = convert_all_fmt(n),
                    Err(_) => continue,
                }
            }
    }

    if lowercase  {
        for i in 0..args.len() {
            args[i] = args[i].to_lowercase();
        }
    } else if uppercase  {
        for i in 0..args.len() {
            args[i] = args[i].to_uppercase();
    }}

    if slugify {
        print!("{}", args.join("-"));
    }
    else if no_space{
        print!("{}", args.join(""));
    }
    else {
        print!("{}", args.join(" "));
    }
}
