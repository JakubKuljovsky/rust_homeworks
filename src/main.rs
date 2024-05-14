use std::io;
use english_numbers::convert_all_fmt;
use std::env;
use std::error::Error;
use std::io::{BufRead, BufReader};
use slugify::slugify;

fn read_from_input() -> Result<String, Box<dyn Error>>
{
    let mut reader = BufReader::new(io::stdin().lock());
    let mut input = String::new();

    loop {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line)?;

        if bytes_read == 0 || line.trim().is_empty() {
            break;
        }

        input.push_str(&line);
    }
    Ok(input)
}

fn csv(text: String) -> Result<String, Box<dyn Error>> {
    let rows: Vec<&str> = text.split("\n").collect();
    
 
    if rows.len() < 1 {
        return Err("No data provided".into());
    }

    let mut head: Vec<&str> = rows[0].split(',').collect();
    let mut cells: Vec<Vec<&str>> = rows[1..].iter().map(|row| row.split(',').collect()).collect();

    let mut max_cells_row = head.len();

    for i in 0..cells.len() - 1 {
        if cells[i].len() > max_cells_row{
            max_cells_row = cells[i].len();
        }
    }

    while head.len() < max_cells_row{
        head.push(" ");
    }

    for i in 0..cells.len() - 1 {
        while cells[i].len() < max_cells_row{
            cells[i].push(" ");
        }
    }

    let mut cells_str: Vec<Vec<String>> = cells.iter().map(|row| row.iter().map(|cell| cell.to_string()).collect()).collect();
    for i in 0..cells.len() - 1{
        let mut max_in_column = 0;
        for j in 0..max_cells_row - 1{
            if max_in_column < cells[i][j].len() {
                max_in_column = cells[i][j].len();
            }
        }


        for j in 0..max_cells_row - 1{
            while cells[i][j].len() < max_cells_row {
                cells_str[i][j] += " ";
            }
        }
    }

    let mut table = String::new();

    
    for (i, field) in head.iter().enumerate() {
        table.push_str(field);
        if i < max_cells_row - 1 {
            table.push_str(" * ");
        }
    }
    table.push_str("\n");

    for row in &cells_str {
        for (i, field) in row.iter().enumerate() {
            table.push_str(field);
            if i < max_cells_row - 1 {
                table.push_str(" * ");
            }
        }
        table.push_str("\n");
    }
    
    Ok(table)
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
    Ok(slugify!(&text, separator = ""))
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {

        let mut text = String::new();
        match read_from_input()
        {
            Ok(input) => {
                text += &input;
            }
            Err(error) => {
                eprintln!("Error in reading lines: {}", error);
                return;
            }
        }

        let final_string = match args[1].as_str() {
            "csv" => csv(text),
            "reverse" => reverse(text),
            "convert-numbers-to-words" => convert_numbers_to_words(text),
            "lowercase" => lowercase(text),
            "uppercase" => uppercase(text),
            "slugify" => slugify_text(text),
            "no-spaces" => no_space(text),
            _ => { Err("Invalid argument".into()) }
        };

        match final_string {
            Ok(text) => {
                println!("{}", text);
            }
            Err(error) => {
                eprintln!("Error : {}", error);
                return;
            }
        }
    }
}


