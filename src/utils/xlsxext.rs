use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::error::Error;
use calamine::{open_workbook, Reader, Xlsx, XlsxError};
use csv::Writer;

pub fn load_workbook(path: &str) -> Result<Xlsx<BufReader<File>>, XlsxError> {
    open_workbook::<Xlsx<_>, _>(path)
}

pub fn read_xlsx_sheet(workbook: &mut Xlsx<BufReader<File>>, sheet_name: &str) -> Result<Vec<Vec<String>>, XlsxError> {
    let range = workbook.worksheet_range(sheet_name)?;
    Ok(range
        .rows()
        .map(|row| row.iter().map(|c| c.to_string()).collect())
        .collect())
}

pub fn get_sheet_names(workbook: &mut Xlsx<BufReader<File>>) -> Vec<String> {
    workbook.sheet_names().to_vec()
}

// For .xlsxcontainer files that contain multiple csv sheets
pub fn write_xlsxcontainer(workbook: &mut Xlsx<BufReader<File>>, output_path: &str,) -> Result<(), Box<dyn Error>> {
    if !Path::new(output_path).is_dir() {
        std::fs::create_dir_all(output_path)?;
    }
    let sheet_names = workbook.sheet_names().to_owned();

    for sheet_name in sheet_names {
        let rows = read_xlsx_sheet(workbook, &sheet_name)?;
        let csv_path = Path::new(output_path)
            .join(format!("{sheet_name}.csv"));
        let mut writer = Writer::from_path(csv_path)?;

        for row in rows {
            writer.write_record(row.iter().map(|s| s.as_str()))?;
        }

        writer.flush()?;
    }
    Ok(())
}

pub fn write_single_xlsx(workbook: &mut Xlsx<BufReader<File>>, sheet_name: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let rows = read_xlsx_sheet(workbook, sheet_name)?;
    let mut writer = Writer::from_path(output_path)?;

    for row in rows {
        writer.write_record(row.iter().map(|s| s.as_str()))?;
    }

    writer.flush()?;
    Ok(())
}
