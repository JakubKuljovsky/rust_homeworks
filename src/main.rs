use std::io;
use english_numbers::convert_all_fmt;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
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

    let lines = io::stdin().lines();
    for line in lines {
        
        let mut words = line.unwrap();
        if lowercase {
            words = words.to_lowercase();
        } else if uppercase {
            words = words.to_uppercase();
        }

        let mut vector: Vec<String> = Vec::new();

        for word in words.split(' ') {
            vector.push(word.to_string());
        }

        if convert_numbers_to_words {
            for i in 0..vector.len() {
                match vector[i].parse::<i64>() {
                    Ok(n) => vector[i] = convert_all_fmt(n),
                    Err(_) => continue,
                }
            }
        }

        if reverse  {
            vector =  vector.clone().into_iter().rev().collect();
        }

        let mut final_string: String = String::new();

        if slugify {
            final_string = vector.join("-");
        }
        else if no_space{
            final_string = vector.join("");
        }
        else {
            final_string = vector.join(" ");
        }

        println!("got a line: {}", final_string);


    }
}
