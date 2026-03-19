use csv;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;

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

fn main() {
    println!("Hello, world!");
}
