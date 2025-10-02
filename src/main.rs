use std::{io::Read};
use cut_rust::parse::{ Range, parse_range, parse_list, parse_delimiter };

enum RunMode { Byte, Character, Field }

fn read_file(file_handle: &str) -> Result<String, Box<std::io::Error>> {
    let mut file = std::fs::File::open(file_handle)?;
    let mut content = String::default();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn byte_mode() {
    todo!();
}

fn char_mode() {
    todo!();
}

fn field_mode() {
    todo!();
}



fn main() -> Result<(), Box<dyn std::error::Error>> {    
    let mut args = std::env::args().skip(1); // skip file name
    
    let mut run_mode: RunMode;
    let mut delimiter: char = '\t';
    let mut list: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-d" => {
                let s = args.next().ok_or_else(|| "expected delimiter after -d".to_string())?;
                delimiter = parse_delimiter(s.as_str()).unwrap();
            }
            "-b" => {
                run_mode = RunMode::Byte;
                list = Some(args.next().ok_or_else(|| "expected list after -b".to_string())?);
            },
            "-c" => {
                run_mode = RunMode::Character;
                list = Some(args.next().ok_or_else(|| "expected list after -c".to_string())?);
            },
            "-f" => {
                run_mode = RunMode::Field;
                list = Some(args.next().ok_or_else(|| "expected list after -f".to_string())?);
            },
            _ => {
                todo!();
            }
        }
    }

    let range: Range;
    match list {
        Some(val) if val.contains("-") => {
            range = parse_range(list.unwrap().as_str())?;
        },
        Some(val) => {
        
        },
        None => println!("no value"),
    }
    
    // let content = read_file("data/fourchords.csv")?;

    // println!("{}", content);

    Ok(())
}
