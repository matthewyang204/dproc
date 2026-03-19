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

fn main() {
    println!("Hello, world!");
}
