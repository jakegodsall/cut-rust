use std::{intrinsics::simd, io::Read};


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
    let mut delimiter: Option<char> = Some('\t');
    let mut range: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-d" => {
                let s = args.next().ok_or_else(|| "expected delimiter after -d".to_string())?;
                let ch = s.as_str().parse::<char>().map_err(|_| format!("delimiter must be exactly one character, got {:?}", s))?;
                delimiter = Some(ch);
            }
            "-b" => {
                run_mode = RunMode::Byte;
                range = Some(args.next().ok_or_else(|| "expected range after -b".to_string())?);
            },
            "-c" => {
                run_mode = RunMode::Character;
                range = Some(args.next().ok_or_else(|| "expected range after -c".to_string())?);
            },
            "-f" => {
                run_mode = RunMode::Field;
                range = Some(args.next().ok_or_else(|| "expected range after -f".to_string())?);
            },
            "_" => {

            }
        }
    }
    
    let content = read_file("data/fourchords.csv")?;

    println!("{}", content);

    Ok(())
}
