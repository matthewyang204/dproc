use calamine::{open_workbook, Reader, Xlsx};

pub fn load_workbook(path: &str) -> Result<Xlsx<calamine::Xlsx<calamine::XlsxReader>>, calamine::XlsxError> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    Ok(workbook)
}


