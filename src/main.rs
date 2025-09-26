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
    let mut list: Option<String> = None;

    let mut run_mode: RunMode;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-b" => {
                run_mode = RunMode::Byte;
                list = args.next();
                if list.is_none() {
                    return Err("expected list after -b".into());
                }
            },
            "-c" => {
                run_mode = RunMode::Character;
                list = args.next();
                if list.is_none() {
                    return Err("expected list after -c".into());
                }
            },
            "-f" => {
                run_mode = RunMode::Field;
                list = args.next();
                if list.is_none() {
                    return Err("expected list after -f".into());
                }
            }
        }
    }
    
    let content = read_file("data/fourchords.csv")?;

    println!("{}", content);

    Ok(())
}
