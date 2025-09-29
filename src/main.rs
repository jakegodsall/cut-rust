use std::{io::Read};


enum RunMode { Byte, Character, Field }
struct Range {
    start: Option<usize>,
    end: Option<usize>,
}

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

fn parse_range(range: String) -> Result<Range, Box<dyn std::error::Error>> {
    Ok(Range {start: Some(1) , end: Some(2)})
}

fn parse_list(list: String) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let nums: Result<Vec<i32>, _> = list
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::parse::<i32>)
        .collect();

    Ok(nums?)
}

fn parse_range(input: &str) -> Result<Range, &'static str> {
    let s = input.trim();
    if s.is_empty() {
        return Err("empty string");
    }

    if s.matches('-').count() > 1 {
        return Err("too many '-' characters");
    }

    if let Some((left, right)) = s.split_once('-') {
        let start = if left.trim().is_empty() {
            None
        } else {
            Some(left.trim().parse::<usize>()
                .map_err(|_| "invalid start")?)
        };

        let end = if right.trim().is_empty() {
            None
        } else {
            Some(right.trim().parse::<usize>()
                .map_err(|_| "invalid end")?)
        };

        Ok(Range { start, end })
    } else {
        let n = s.parse::<usize>().map_err(|_| "invalid numbrer")?;
        Ok(Range {
            start: Some(n),
            end: Some(n)
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {    
    let mut args = std::env::args().skip(1); // skip file name
    
    let mut run_mode: RunMode;
    let mut delimiter: Option<char> = Some('\t');
    let mut list: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-d" => {
                let s = args.next().ok_or_else(|| "expected delimiter after -d".to_string())?;
                let ch = s.as_str().parse::<char>().map_err(|_| format!("delimiter must be exactly one character, got {:?}", s))?;
                delimiter = Some(ch);
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

        },
        Some(val) => {
            range = match parse_range(val) {
                
            }
        },
        None => println!("no value"),
    }
    
    // let content = read_file("data/fourchords.csv")?;

    // println!("{}", content);

    Ok(())
}
