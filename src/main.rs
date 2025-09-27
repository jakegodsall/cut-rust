use std::io::Read;


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
    let mut args = std::env::args().skip(1);
    
    let mut run_mode: RunMode;
    let mut delimiter: Option<char> = None;
    let mut range: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-d" => {
                delimiter = args.next();
                if delimiter.is_none() {
                    return Err("expected delimiter after -d".into());
                }
            }
            "-b" => {
                run_mode = RunMode::Byte;
                range = args.next();
                if range.is_none() {
                    return Err("expected range after -b".into());
                }
            },
            "-c" => {
                run_mode = RunMode::Character;
                range = args.next();
                if range.is_none() {
                    return Err("expected range after -c".into());
                }
            },
            "-f" => {
                run_mode = RunMode::Field;
                range = args.next();
                if range.is_none() {
                    return Err("expected range after -f".into());
                }
            },
            "_" => {

            }
        }
    }
    
    let content = read_file("data/fourchords.csv")?;

    println!("{}", content);

    Ok(())
}
