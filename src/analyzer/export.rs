use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

/// Export functionality for analysis results - ye class results export karta hai
pub struct DataExporter;

impl DataExporter {
    pub fn new() -> Self {
        Self
    }
    
    //yaha pe badme XML export bhi add karna ha
    pub fn export_to_json<T: Serialize>(&self, data: &T, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(data)?;
        let mut file = File::create(filename)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
    
    pub fn export_to_csv(&self, data: &[Vec<String>], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(filename)?;
        
        for row in data {
            let csv_line = row.join(",");
            writeln!(file, "{}", csv_line)?;
        }
        
        Ok(())
    }
    
    //function export_to_xml() {}
    //function export_to_sql() {}
}
