// Standard library imports
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::env;
use std::path::Path;
use std::ffi::OsStr;

pub fn read_space_delimited_values(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut values = Vec::new();
    let file = File::open(file_path)?;
    let reader = std::io::BufReader::new(file);
    for line in reader.lines(){
        match line {
            Ok(content) => values.push(content),
            Err(e) => eprintln!("ERROR: Error reading line from stdin: {}", e),
        }
    }
    Ok(values)
}

pub fn write_space_delimited_values(file_path: &str, values: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(file_path)?;
    let line = values.join(" ");
    use std::io::Write;
    file.write_all(line.as_bytes())?;
    Ok(())
}
