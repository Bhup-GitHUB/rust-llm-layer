use serde::{Deserialize, Serialize};

/// Database query log entry - yaha hum query ka complete record store karte hain
/// This struct captures all the important metrics for a database query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryLog {
    /// The actual SQL query string
    pub query: String,
    /// Execution time in milliseconds - ye dekhna hai kitna time laga
    pub execution_time_ms: u64,
    /// Unix timestamp when query was executed
    pub timestamp: u64,
    /// List of tables accessed by this query
    pub tables_accessed: Vec<String>,
    /// Number of rows scanned during execution
    pub rows_scanned: u64,
}

impl QueryLog {
    /// Creates a new QueryLog instance - constructor function hai ye
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

    /// Determines the type of SQL query - SELECT, INSERT, UPDATE, DELETE, etc.
    /// Ye function query ka type identify karta hai
    pub fn query_type(&self) -> String {
        let query_upper = self.query.trim().to_uppercase();
        
        // Simple pattern matching for common SQL operations
        if query_upper.starts_with("SELECT") {
            "SELECT".to_string()
        } else if query_upper.starts_with("INSERT") {
            "INSERT".to_string()
        } else if query_upper.starts_with("UPDATE") {
            "UPDATE".to_string()
        } else if query_upper.starts_with("DELETE") {
            "DELETE".to_string()
        } else {
            "OTHER".to_string() // koi aur type ka query hai
        }
    }

    /// Checks if this query is considered slow based on threshold
    /// threshold se compare karke slow query hai ya nahi check karta hai
    pub fn is_slow(&self, threshold_ms: u64) -> bool {
        self.execution_time_ms > threshold_ms
    }

    /// Calculates the efficiency score based on rows scanned vs execution time
    /// ye function efficiency calculate karta hai - important hai ye method
    pub fn efficiency_score(&self) -> f64 {
        if self.rows_scanned == 0 {
            return 1.0; // perfect efficiency if no rows scanned
        }
        
        let time_per_row = self.execution_time_ms as f64 / self.rows_scanned as f64;
        // Lower time per row = better efficiency
        1000.0 / (time_per_row + 1.0) // normalize to 0-1000 range
    }
}
