use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

pub fn read_json_file<T: for<'de> Deserialize<'de>>(path: &str) -> Result<T, String> {
    let path = PathBuf::from(path);
    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&data).map_err(|e| e.to_string())
}