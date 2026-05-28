// Standard library imports
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::env;
use std::path::Path;
use std::ffi::OsStr;

// Load crates
use csv;

pub fn read_csv_matrix(path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();

    for result in rdr.records() {
        let record = result?;
        rows.push(record.iter().map(|s| s.to_string()).collect());
    }

    Ok(rows)
}

pub fn modify_column(
    rows: &mut Vec<Vec<String>>,
    col: usize,
    new_values: &[String]
) {
    for (i, row) in rows.iter_mut().enumerate() {
        if i < new_values.len() && col < row.len() {
            row[col] = new_values[i].clone();
        }
    }
}

pub fn modify_row(
    rows: &mut Vec<Vec<String>>,
    row_index: usize,
    new_values: &[String]
) {
    if row_index < rows.len() {
        rows[row_index] = new_values.to_vec();
    }
}

pub fn write_csv_matrix(path: &str, rows: &[Vec<String>]) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(path)?;

    for row in rows {
        wtr.write_record(row)?;
    }

    wtr.flush()?;
    Ok(())
}
