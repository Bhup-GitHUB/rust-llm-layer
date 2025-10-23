use super::QueryLog;
use std::collections::HashMap;

/// Represents a pattern found in query logs - ye struct query patterns ko represent karta hai
/// This helps identify common query behaviors and performance issues
#[derive(Debug, Clone)]
pub struct QueryPattern {
    /// Type of SQL operation (SELECT, INSERT, etc.)
    pub query_type: String,
    /// Average execution time for this pattern
    pub avg_execution_time_ms: f64,
    /// How many times this pattern occurred
    pub frequency: u64,
    /// Tables involved in this pattern
    pub tables: Vec<String>,
    /// Combined score indicating how problematic this pattern is
    pub slowness_score: f64,
    /// Total rows scanned across all occurrences
    pub total_rows_scanned: u64,
}

/// Analyzes query logs to find patterns and performance issues
/// Ye class query logs ko analyze karke patterns find karta hai
pub struct PatternAnalyzer {
    logs: Vec<QueryLog>,
}

impl PatternAnalyzer {
    /// Creates a new PatternAnalyzer instance
    pub fn new() -> Self {
        Self { logs: Vec::new() }
    }

    /// Adds a single query log to the analyzer
    /// Ek query log add karta hai analyzer mein
    pub fn add_log(&mut self, log: QueryLog) {
        self.logs.push(log);
    }

    /// Adds multiple query logs at once - bulk add karta hai
    pub fn add_logs(&mut self, logs: Vec<QueryLog>) {
        self.logs.extend(logs);
    }

    /// Analyzes all logs and returns discovered patterns
    /// Sabhi logs ko analyze karke patterns return karta hai
    pub fn analyze(&self) -> Vec<QueryPattern> {
        let mut patterns: HashMap<String, Vec<&QueryLog>> = HashMap::new();

        // Group logs by query type - query type ke basis pe group karta hai
        for log in &self.logs {
            let query_type = log.query_type();
            patterns.entry(query_type).or_insert_with(Vec::new).push(log);
        }

        // Convert grouped logs into QueryPattern structs
        patterns
            .into_iter()
            .map(|(query_type, logs)| {
                // Calculate statistics for this pattern
                let total_time: u64 = logs.iter().map(|l| l.execution_time_ms).sum();
                let total_rows: u64 = logs.iter().map(|l| l.rows_scanned).sum();
                let avg_time = total_time as f64 / logs.len() as f64;
                
                // Slowness score combines frequency and average time
                let slowness_score = avg_time * logs.len() as f64;

                // Collect and deduplicate table names
                let mut tables: Vec<String> = logs
                    .iter()
                    .flat_map(|l| l.tables_accessed.clone())
                    .collect();
                tables.sort();
                tables.dedup();

                QueryPattern {
                    query_type,
                    avg_execution_time_ms: avg_time,
                    frequency: logs.len() as u64,
                    tables,
                    slowness_score,
                    total_rows_scanned: total_rows,
                }
            })
            .collect()
    }

    /// Returns the top N slowest patterns - sabse slow patterns return karta hai
    pub fn get_slow_patterns(&self, n: usize) -> Vec<QueryPattern> {
        let mut patterns = self.analyze();
        // Sort by slowness score in descending order
        patterns.sort_by(|a, b| b.slowness_score.partial_cmp(&a.slowness_score).unwrap());
        patterns.into_iter().take(n).collect()
    }

    /// Returns patterns that occur at least min_frequency times
    /// Jo patterns minimum frequency se zyada baar aaye hain
    pub fn get_frequent_patterns(&self, min_frequency: u64) -> Vec<QueryPattern> {
        self.analyze()
            .into_iter()
            .filter(|p| p.frequency >= min_frequency)
            .collect()
    }

    /// Returns total number of queries analyzed
    /// Total kitne queries analyze kiye hain
    pub fn total_queries(&self) -> usize {
        self.logs.len()
    }

    /// Clears all stored logs - sabhi logs clear kar deta hai
    pub fn clear(&mut self) {
        self.logs.clear();
    }

    /// Gets performance summary for all patterns
    /// Sabhi patterns ka performance summary deta hai
    pub fn get_performance_summary(&self) -> (f64, f64, usize) {
        let patterns = self.analyze();
        let total_queries = self.total_queries();
        
        if total_queries == 0 {
            return (0.0, 0.0, 0);
        }

        let avg_time: f64 = patterns.iter()
            .map(|p| p.avg_execution_time_ms * p.frequency as f64)
            .sum::<f64>() / total_queries as f64;

        let slow_queries = patterns.iter()
            .filter(|p| p.avg_execution_time_ms > 100.0)
            .map(|p| p.frequency)
            .sum::<u64>() as usize;

        (avg_time, patterns.len() as f64, slow_queries)
    }
}
