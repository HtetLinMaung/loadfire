use std::{collections::HashMap, path::Path};

use calamine::{open_workbook_auto, Reader};

pub fn load_data(
    file_path: &str,
) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    let extension = path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("");

    match extension.to_lowercase().as_str() {
        "csv" => load_csv_data(file_path),
        "xls" | "xlsx" => load_excel_data(file_path),
        _ => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Unsupported file format",
        ))),
    }
}

pub fn load_csv_data(
    file_path: &str,
) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let headers = rdr.headers()?.clone();

    let mut data = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let mut row_data = HashMap::new();
        for (header, field) in headers.iter().zip(record.iter()) {
            row_data.insert(header.to_string(), field.to_string());
        }
        data.push(row_data);
    }

    Ok(data)
}

pub fn load_excel_data(
    file_path: &str,
) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
    let mut workbook = open_workbook_auto(file_path)?;
    let range = workbook
        .worksheet_range_at(0)
        .ok_or("Cannot find worksheet")??;

    let headers: Vec<String> = range
        .rows()
        .next()
        .unwrap_or_default()
        .iter()
        .map(|c| c.get_string().unwrap_or_default().to_string())
        .collect();

    let mut data = Vec::new();
    for row in range.rows().skip(1) {
        let mut row_data = HashMap::new();
        for (idx, cell) in row.iter().enumerate() {
            let value = cell.to_string(); // handle conversion as needed
            if idx < headers.len() {
                row_data.insert(headers[idx].clone(), value);
            }
        }
        data.push(row_data);
    }

    Ok(data)
}
