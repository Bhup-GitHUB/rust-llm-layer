use std::collections::VecDeque;
use serde::{Deserialize, Serialize};

/// Anomaly detection result - ye struct anomaly detection result store karta hai
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyResult {
    pub is_anomaly: bool,
    pub anomaly_type: String, // "sudden_slow", "unusual_pattern", "spike"
    pub severity: f64, // 0.0 to 1.0
    pub description: String,
    pub baseline_value: f64,
    pub current_value: f64,
}

/// Detects anomalies in query performance - ye class anomalies detect karta hai
pub struct AnomalyDetector {
    execution_times: VecDeque<u64>,
    max_history: usize,
    baseline_threshold: f64,
}

impl AnomalyDetector {
    pub fn new() -> Self {
        Self {
            execution_times: VecDeque::new(),
            max_history: 100, // Keep last 100 queries for analysis
            baseline_threshold: 2.0, // 2x baseline is considered anomaly
        }
    }
    
    /// Add query execution time for analysis - ye method query execution time add karta hai
    pub fn add_execution_time(&mut self, execution_time: u64) {
        self.execution_times.push_back(execution_time);
        
        // Keep only recent history - ye recent history maintain karta hai
        if self.execution_times.len() > self.max_history {
            self.execution_times.pop_front();
        }
    }
    
    /// Detect anomalies in current query - ye method current query mein anomalies detect karta hai
    pub fn detect_anomaly(&self, current_time: u64) -> AnomalyResult {
        if self.execution_times.len() < 10 {
            // Not enough data for analysis - ye insufficient data ke liye hai
            return AnomalyResult {
                is_anomaly: false,
                anomaly_type: "insufficient_data".to_string(),
                severity: 0.0,
                description: "Not enough historical data for anomaly detection".to_string(),
                baseline_value: 0.0,
                current_value: current_time as f64,
            };
        }
        
        let baseline = self.calculate_baseline();
        let deviation = (current_time as f64 - baseline) / baseline;
        
        // Check for sudden slowdown - ye sudden slowdown check karta hai
        if deviation > self.baseline_threshold {
            return AnomalyResult {
                is_anomaly: true,
                anomaly_type: "sudden_slow".to_string(),
                severity: (deviation / self.baseline_threshold).min(1.0),
                description: format!(
                    "Query execution time increased by {:.1}% from baseline",
                    deviation * 100.0
                ),
                baseline_value: baseline,
                current_value: current_time as f64,
            };
        }
        
        // Check for unusual patterns - ye unusual patterns check karta hai
        if self.detect_unusual_pattern(current_time) {
            return AnomalyResult {
                is_anomaly: true,
                anomaly_type: "unusual_pattern".to_string(),
                severity: 0.7,
                description: "Unusual execution pattern detected".to_string(),
                baseline_value: baseline,
                current_value: current_time as f64,
            };
        }
        
        // No anomaly detected - ye no anomaly case hai
        AnomalyResult {
            is_anomaly: false,
            anomaly_type: "normal".to_string(),
            severity: 0.0,
            description: "Query performance within normal range".to_string(),
            baseline_value: baseline,
            current_value: current_time as f64,
        }
    }
    
    fn calculate_baseline(&self) -> f64 {
        // Calculate baseline using median - ye median se baseline calculate karta hai
        let mut times: Vec<u64> = self.execution_times.iter().cloned().collect();
        times.sort();
        
        let len = times.len();
        if len % 2 == 0 {
            (times[len / 2 - 1] + times[len / 2]) as f64 / 2.0
        } else {
            times[len / 2] as f64
        }
    }
    
    fn detect_unusual_pattern(&self, current_time: u64) -> bool {
        // Simple pattern detection - ye basic pattern detection hai
        let recent_avg: f64 = self.execution_times.iter()
            .rev()
            .take(5)
            .map(|&t| t as f64)
            .sum::<f64>() / 5.0;
            
        let older_avg: f64 = self.execution_times.iter()
            .rev()
            .skip(5)
            .take(10)
            .map(|&t| t as f64)
            .sum::<f64>() / 10.0;
            
        // Check for significant change in trend - ye trend change check karta hai
        if older_avg > 0.0 {
            let trend_change = (recent_avg - older_avg) / older_avg;
            trend_change.abs() > 0.5 // 50% change is unusual
        } else {
            false
        }
    }
    
    /// Get anomaly summary - ye method anomaly summary deta hai
    pub fn get_anomaly_summary(&self) -> (usize, f64, f64) {
        let total_queries = self.execution_times.len();
        if total_queries == 0 {
            return (0, 0.0, 0.0);
        }
        
        let baseline = self.calculate_baseline();
        let anomalies = self.execution_times.iter()
            .filter(|&&time| {
                let deviation = (time as f64 - baseline) / baseline;
                deviation > self.baseline_threshold
            })
            .count();
            
        let anomaly_rate = anomalies as f64 / total_queries as f64;
        let avg_execution_time = self.execution_times.iter().sum::<u64>() as f64 / total_queries as f64;
        
        (anomalies, anomaly_rate, avg_execution_time)
    }
    
    /// Get performance recommendations - ye method performance recommendations deta hai
    pub fn get_performance_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        let (anomalies, anomaly_rate, avg_time) = self.get_anomaly_summary();
        
        if anomaly_rate > 0.1 {
            recommendations.push(format!(
                "High anomaly rate detected ({:.1}%) - investigate query patterns",
                anomaly_rate * 100.0
            ));
        }
        
        if avg_time > 200.0 {
            recommendations.push("Average execution time is high - consider query optimization".to_string());
        }
        
        if anomalies > 5 {
            recommendations.push("Multiple anomalies detected - review database configuration".to_string());
        }
        
        recommendations
    }
}
