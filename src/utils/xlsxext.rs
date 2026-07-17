use std::fs::File;
use std::io::BufReader;
use calamine::{open_workbook, Reader, Xlsx, XlsxError};

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
