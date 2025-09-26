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
    let options: Vec<String> = std::env::args().collect();
    
    let mut run_mode = RunMode::Byte;
    for option in options.iter() {
        run_mode = match option.as_str() {
            "-b" => RunMode::Byte,
            "-c" => RunMode::Character,
            "-f" => RunMode::Field,
            _ => panic!("Incorrect option"),
        }
    }

    match run_mode {
        RunMode::Byte => byte_mode(),
        RunMode::Character => char_mode(),
        RunMode::Field => field_mode(),
    }
    
    let content = read_file("data/fourchords.csv")?;

    println!("{}", content);

    Ok(())
}
