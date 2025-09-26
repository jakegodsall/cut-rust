use std::io::Read;


enum Mode { Byte, Character, Field }

fn read_file(file_handle: &str) -> Result<String, Box<std::io::Error>> {
    let mut file = std::fs::File::open(file_handle)?;
    let mut content = String::default();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = read_file("data/fourchords.csv")?;

    println!("{}", content);

    Ok(())
}
