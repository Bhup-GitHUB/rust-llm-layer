use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryLog {
    pub query: String,
    pub execution_time_ms: u64,
    pub timestamp: u64,
    pub tables_accessed: Vec<String>,
    pub rows_scanned: u64,
}

impl QueryLog {
    pub fn new(
        query: String,
        execution_time_ms: u64,
        timestamp: u64,
        tables_accessed: Vec<String>,
        rows_scanned: u64,
    ) -> Self {
        Self {
            query,
            execution_time_ms,
            timestamp,
            tables_accessed,
            rows_scanned,
        }
    }

    pub fn query_type(&self) -> String {
        let query_upper = self.query.trim().to_uppercase();
        if query_upper.starts_with("SELECT") {
            "SELECT".to_string()
        } else if query_upper.starts_with("INSERT") {
            "INSERT".to_string()
        } else if query_upper.starts_with("UPDATE") {
            "UPDATE".to_string()
        } else if query_upper.starts_with("DELETE") {
            "DELETE".to_string()
        } else {
            "OTHER".to_string()
        }
    }

    pub fn is_slow(&self, threshold_ms: u64) -> bool {
        self.execution_time_ms > threshold_ms
    }
}
