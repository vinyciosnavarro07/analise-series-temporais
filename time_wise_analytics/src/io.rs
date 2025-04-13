use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct TimeSeriesData {
    pub timestamp: String,
    pub value: f64,
}

/// Lê dados de um arquivo CSV e retorna um vetor de `TimeSeriesData`.
pub fn read_csv(file_path: &str) -> Result<Vec<TimeSeriesData>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(file_path)?;
    let mut data = Vec::new();

    for result in reader.deserialize() {
        let record: TimeSeriesData = result?;
        data.push(record);
    }

    Ok(data)
}

/// Lê dados de um arquivo JSON e retorna um vetor de `TimeSeriesData`.
pub fn read_json(file_path: &str) -> Result<Vec<TimeSeriesData>, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_path)?;
    let data: Vec<TimeSeriesData> = serde_json::from_str(&file_content)?;
    Ok(data)
}

/// Lê dados de um arquivo (CSV ou JSON) e retorna um vetor de `TimeSeriesData`.
pub fn read_file(file_path: &str) -> Result<Vec<TimeSeriesData>, Box<dyn Error>> {
    if file_path.ends_with(".csv") {
        read_csv(file_path)
    } else if file_path.ends_with(".json") {
        read_json(file_path)
    } else {
        Err("Formato de arquivo não suportado. Use .csv ou .json.".into())
    }
}