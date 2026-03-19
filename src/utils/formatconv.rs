use csv;
use std::error::Error;
use std::fs::File;

fn read_csv_column(file_path: &str, column_name: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut column_data = Vec::new();
    let headers = rdr.headers()?.clone();
    let results = rdr.records();
    let col_index = headers.iter().position(|h| h == column_name).unwrap();

    for result in results {
        let record = result?;
        if let Some(value) = record.get(col_index) {
            column_data.push(value.to_string());
        }
    }

    Ok(column_data)
}

fn main() {
    println!("Hello, world!");
}
