use super::QueryLog;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct QueryPattern {
    pub query_type: String,
    pub avg_execution_time_ms: f64,
    pub frequency: u64,
    pub tables: Vec<String>,
    pub slowness_score: f64,
    pub total_rows_scanned: u64,
}

pub struct PatternAnalyzer {
    logs: Vec<QueryLog>,
}

impl PatternAnalyzer {
    pub fn new() -> Self {
        Self { logs: Vec::new() }
    }

    pub fn add_log(&mut self, log: QueryLog) {
        self.logs.push(log);
    }

    pub fn add_logs(&mut self, logs: Vec<QueryLog>) {
        self.logs.extend(logs);
    }

    pub fn analyze(&self) -> Vec<QueryPattern> {
        let mut patterns: HashMap<String, Vec<&QueryLog>> = HashMap::new();

        for log in &self.logs {
            let query_type = log.query_type();
            patterns.entry(query_type).or_insert_with(Vec::new).push(log);
        }

        patterns
            .into_iter()
            .map(|(query_type, logs)| {
                let total_time: u64 = logs.iter().map(|l| l.execution_time_ms).sum();
                let total_rows: u64 = logs.iter().map(|l| l.rows_scanned).sum();
                let avg_time = total_time as f64 / logs.len() as f64;
                
                let slowness_score = avg_time * logs.len() as f64;

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

    pub fn get_slow_patterns(&self, n: usize) -> Vec<QueryPattern> {
        let mut patterns = self.analyze();
        patterns.sort_by(|a, b| b.slowness_score.partial_cmp(&a.slowness_score).unwrap());
        patterns.into_iter().take(n).collect()
    }

    pub fn get_frequent_patterns(&self, min_frequency: u64) -> Vec<QueryPattern> {
        self.analyze()
            .into_iter()
            .filter(|p| p.frequency >= min_frequency)
            .collect()
    }

    pub fn total_queries(&self) -> usize {
        self.logs.len()
    }

    pub fn clear(&mut self) {
        self.logs.clear();
    }

    //yaha pe badme machine learning model bhi add karna ha
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
