use crate::analyzer::QueryLog;

/// Performance prediction result - ye struct prediction ka result store karta hai
#[derive(Debug, Clone)]
pub struct PerformancePrediction {
    pub estimated_time_ms: u64,
    pub confidence: f64,
    pub recommendation: String,
}

/// Predicts query performance based on historical data - historical data se performance predict karta hai
pub struct PerformancePredictor {
    historical_data: Vec<QueryLog>,
    cache_enabled: bool,
}

impl PerformancePredictor {
    pub fn new(cache_enabled: bool) -> Self {
        Self {
            historical_data: Vec::new(),
            cache_enabled,
        }
    }

    pub fn add_historical_data(&mut self, log: QueryLog) {
        self.historical_data.push(log);
    }

    pub fn predict(&self, query_type: &str, rows_to_scan: u64) -> PerformancePrediction {
        let similar_queries: Vec<&QueryLog> = self
            .historical_data
            .iter()
            .filter(|log| log.query_type() == query_type)
            .collect();

        if similar_queries.is_empty() {
            return PerformancePrediction {
                estimated_time_ms: self.estimate_baseline(rows_to_scan),
                confidence: 0.3,
                recommendation: "No historical data available".to_string(),
            };
        }

        let avg_time: u64 = similar_queries.iter().map(|l| l.execution_time_ms).sum::<u64>()
            / similar_queries.len() as u64;

        let row_factor = (rows_to_scan as f64 / 1000.0).max(1.0);
        let estimated_time = (avg_time as f64 * row_factor) as u64;

        let cache_factor = if self.cache_enabled { 0.6 } else { 1.0 };
        let final_estimate = (estimated_time as f64 * cache_factor) as u64;

        let confidence = (similar_queries.len() as f64 / 10.0).min(0.95);

        let recommendation = if final_estimate > 100 {
            "Consider adding index or optimizing query".to_string()
        } else {
            "Query performance looks good".to_string()
        };

        PerformancePrediction {
            estimated_time_ms: final_estimate,
            confidence,
            recommendation,
        }
    }

    fn estimate_baseline(&self, rows_to_scan: u64) -> u64 {
        let base_time = 10;
        base_time + (rows_to_scan / 100)
    }

    pub fn enable_cache(&mut self) {
        self.cache_enabled = true;
    }

    pub fn disable_cache(&mut self) {
        self.cache_enabled = false;
    }
}
