// Standard library imports
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::env;

// Load crates
use csv;

fn read_csv_column(file_path: &str, column_number: &f64) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut column_data = Vec::new();
    let headers = rdr.headers()?.clone();
    let results = rdr.records();
    let col_index = *column_number as usize;

    for result in results {
        if let Some(value) = result?.get(col_index) {
            column_data.push(value.to_string());
        }
    }

    Ok(column_data)
}

fn write_csv_column(file_path: &str, column_number: &f64, data: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = csv::Writer::from_path(file_path)?;
    let col_index = *column_number as usize;
    for value in data {
        wtr.write_record(&[value])?;
    }
    wtr.flush()?;
    Ok(())
}

fn read_csv_row(file_path: &str, row_number: &f64) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut row_data = Vec::new();
    let results = rdr.records();
    let row_index = *row_number as usize;

    if *row_number == 0.0 {
        return Ok(rdr.headers()?.iter().map(|s| s.to_string()).collect());
    }

    for (i, result) in results.enumerate() {
        if i == row_index {
            let record = result?;
            for value in record.iter() {
                row_data.push(value.to_string());
            }
            break;
        }
    }

    Ok(row_data)
}

fn write_csv_row(file_path: &str, row_number: &f64, data: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = csv::Writer::from_path(file_path)?;
    let row_index = *row_number as usize;
    for (i, value) in data.iter().enumerate() {
        if i == row_index {
            wtr.write_record(&[value])?;
        }
    }
    wtr.flush()?;
    Ok(())
}

fn read_space_delimited_values(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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

fn write_space_delimited_values(file_path: &str, values: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(file_path)?;
    let line = values.join(" ");
    use std::io::Write;
    file.write_all(line.as_bytes())?;
    Ok(())
}

fn getArgs() -> Vec<String> {
	let args: Vec<String> = env::args().collect();
	return args;
}

fn main() {
    println!("WARNING: Unimplemented function");

    let args = getArgs();
    
}
