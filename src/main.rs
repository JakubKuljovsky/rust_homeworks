use std::io;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;
use std::thread;
use std::process;
use std::error::Error;
use prettytable::Table;
use english_numbers::convert_all_fmt;
use slugify::slugify;
use std::env;


fn csv(text: String) -> Result<String, Box<dyn Error>> {
    let table = Table::from_csv_file(text)?;
    Ok(table.to_string())
}


fn reverse(text: String) -> Result<String, Box<dyn Error>>
{
    Ok(text.split(|c: char| c.is_whitespace()  && c != '\n'  && c != '\t')
         .filter(|s| !s.is_empty())
         .rev()
         .collect::<Vec<&str>>()
         .join(" "))

}

fn convert_numbers_to_words(text: String) -> Result<String, Box<dyn Error>>
{
    Ok(text.split(|c: char| c.is_whitespace()  && c != '\n'  && c != '\t')
         .filter(|s| !s.is_empty())
         .map(|word| {
             match word.parse::<i64>() {
                 Ok(n) => convert_all_fmt(n),
                 Err(_) => word.to_string(),
             }
         })
         .collect::<Vec<String>>()
         .join(" "))
}

fn lowercase(text: String) -> Result<String, Box<dyn Error>>
{
    Ok(text.to_lowercase())
}

fn uppercase(text: String) -> Result<String, Box<dyn Error>>
{
    Ok(text.to_uppercase())
}

fn slugify_text(text: String) -> Result<String, Box<dyn Error>>
{
    Ok(slugify!(&text, separator = "-"))
}

fn no_space(text: String) -> Result<String, Box<dyn Error>>
{
    let no_space_string: String = text.split(|c: char| c.is_whitespace()).collect::<Vec<&str>>().join("");
    Ok(no_space_string)
}


fn read_from_input(sender: mpsc::Sender<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = BufReader::new(io::stdin().lock());


    loop {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line)?;

        if bytes_read == 0 || line.trim().is_empty() {
            break;
        }

        sender.send(line.trim().to_string())?;
    }

    Ok(())
}

fn process_input(text: String) {
    let mut iter = text.splitn(2, ' ');
    let command = if let Some(cmd) = iter.next() {
        cmd
    } else {
        eprintln!("Error: Expected command");
        return;
    };

    let input = if let Some(inp) = iter.next() {
        inp.to_string()
    } else {
        eprintln!("Error: Expected input");
        return;
    };

    let number: u32 = match command.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let final_string : Result<String, Box<dyn Error>>;

    if number > 0 {
        final_string = match number {
            1 => csv(input),
            2 => reverse(input),
            3 => convert_numbers_to_words(input),
            4 => lowercase(input),
            5 => uppercase(input),
            6 => slugify_text(input),
            7 => no_space(input),
            _ => Ok(input)
        };
    }
    else {
        final_string = match command {
            "csv" => csv(text),
            "reverse" => reverse(text),
            "convert-numbers-to-words" => convert_numbers_to_words(text),
            "lowercase" => lowercase(text),
            "uppercase" => uppercase(text),
            "slugify" => slugify_text(text),
            "no-spaces" => no_space(text),
            _ => Ok(input)
        };
    }

    match final_string {
        Ok(text) => {
            println!("{}", text);
        }
        Err(error) => {
            eprintln!("Error : {}", error);
            process::exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
    let (tx, rx) = mpsc::channel();

    let read_handle = thread::spawn(move || {
        match read_from_input(tx) {
            Err(error) => {
                eprintln!("{}", error);
                process::exit(1);
            }
            Ok(_) => {} 
        }
    });

    let process_handle = thread::spawn(move || {
        loop {
            match rx.try_recv() {
                Ok(input) => {
                    process_input(input);
                }
                Err(mpsc::TryRecvError::Empty) => {

                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    break;
                }
            }
        }
        let input = rx.recv().unwrap();
        process_input(input);
    });

    read_handle.join().unwrap();
    process_handle.join().unwrap();
}
else {
    process_input(args[1..].join(" "));
}



}