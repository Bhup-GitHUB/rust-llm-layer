use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetric {
    pub timestamp: u64,
    pub metric_name: String,
    pub value: f64,
    pub unit: String,
    pub severity: String, // "normal", "warning", "critical"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    pub alert_id: String,
    pub metric_name: String,
    pub current_value: f64,
    pub threshold_value: f64,
    pub severity: String,
    pub message: String,
    pub timestamp: u64,
}

pub struct PerformanceMonitor {
    metrics_history: HashMap<String, Vec<PerformanceMetric>>,
    alert_thresholds: HashMap<String, f64>,
    max_history_size: usize,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        let mut thresholds = HashMap::new();
        thresholds.insert("query_time".to_string(), 1000.0); // 1 second
        thresholds.insert("cpu_usage".to_string(), 80.0); // 80%
        thresholds.insert("memory_usage".to_string(), 90.0); // 90%
        thresholds.insert("connection_count".to_string(), 100.0); // 100 connections
        
        Self {
            metrics_history: HashMap::new(),
            alert_thresholds: thresholds,
            max_history_size: 1000,
        }
    }
    
    //yaha pe badme real-time streaming bhi add karna ha
    pub fn record_metric(&mut self, metric: PerformanceMetric) {
        let metric_name = metric.metric_name.clone();
        let entry = self.metrics_history.entry(metric_name).or_insert_with(Vec::new);
        
        entry.push(metric);
        
        // Keep only recent history
        if entry.len() > self.max_history_size {
            entry.remove(0);
        }
    }
    
    pub fn check_alerts(&self) -> Vec<PerformanceAlert> {
        let mut alerts = Vec::new();
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        for (metric_name, metrics) in &self.metrics_history {
            if let Some(threshold) = self.alert_thresholds.get(metric_name) {
                if let Some(latest_metric) = metrics.last() {
                    if latest_metric.value > *threshold {
                        alerts.push(PerformanceAlert {
                            alert_id: format!("{}_{}", metric_name, current_time),
                            metric_name: metric_name.clone(),
                            current_value: latest_metric.value,
                            threshold_value: *threshold,
                            severity: self.determine_severity(latest_metric.value, *threshold),
                            message: format!(
                                "{} exceeded threshold: {} > {}",
                                metric_name, latest_metric.value, threshold
                            ),
                            timestamp: current_time,
                        });
                    }
                }
            }
        }
        
        alerts
    }
    
    fn determine_severity(&self, current_value: f64, threshold: f64) -> String {
        let ratio = current_value / threshold;
        if ratio > 2.0 {
            "critical".to_string()
        } else if ratio > 1.5 {
            "warning".to_string()
        } else {
            "normal".to_string()
        }
    }
    
    pub fn get_performance_trends(&self) -> HashMap<String, String> {
        let mut trends = HashMap::new();
        
        for (metric_name, metrics) in &self.metrics_history {
            if metrics.len() >= 10 {
                let trend = self.calculate_trend(metrics);
                trends.insert(metric_name.clone(), trend);
            }
        }
        
        trends
    }
    
    fn calculate_trend(&self, metrics: &[PerformanceMetric]) -> String {
        if metrics.len() < 10 {
            return "insufficient_data".to_string();
        }
        
        let recent_avg: f64 = metrics.iter().rev().take(5).map(|m| m.value).sum::<f64>() / 5.0;
        let older_avg: f64 = metrics.iter().rev().skip(5).take(5).map(|m| m.value).sum::<f64>() / 5.0;
        
        let change_percent = ((recent_avg - older_avg) / older_avg) * 100.0;
        
        if change_percent > 20.0 {
            "increasing".to_string()
        } else if change_percent < -20.0 {
            "decreasing".to_string()
        } else {
            "stable".to_string()
        }
    }
    
    pub fn get_performance_summary(&self) -> (f64, usize, usize) {
        let total_metrics: usize = self.metrics_history.values().map(|v| v.len()).sum();
        let active_alerts = self.check_alerts().len();
        let critical_alerts = self.check_alerts().iter()
            .filter(|alert| alert.severity == "critical")
            .count();
            
        let health_score = if total_metrics > 0 {
            let avg_health = self.metrics_history.values()
                .map(|metrics| {
                    let normal_count = metrics.iter().filter(|m| m.severity == "normal").count();
                    (normal_count as f64 / metrics.len() as f64) * 100.0
                })
                .sum::<f64>() / self.metrics_history.len() as f64;
            avg_health
        } else {
            0.0
        };
        
        (health_score, active_alerts, critical_alerts)
    }
    
    //yaha pe badme predictive analysis bhi add karna ha
    pub fn predict_performance(&self, metric_name: &str, hours_ahead: u64) -> Option<f64> {
        if let Some(metrics) = self.metrics_history.get(metric_name) {
            if metrics.len() >= 20 {
                let recent_values: Vec<f64> = metrics.iter().rev().take(20).map(|m| m.value).collect();
                let trend = self.calculate_linear_trend(&recent_values);
                let current_value = recent_values[0];
                let predicted_value = current_value + (trend * hours_ahead as f64);
                Some(predicted_value)
            } else {
                None
            }
        } else {
            None
        }
    }
    
    fn calculate_linear_trend(&self, values: &[f64]) -> f64 {
        if values.len() < 2 {
            return 0.0;
        }
        
        let n = values.len() as f64;
        let sum_x: f64 = (0..values.len()).map(|i| i as f64).sum();
        let sum_y: f64 = values.iter().sum();
        let sum_xy: f64 = values.iter().enumerate().map(|(i, v)| i as f64 * v).sum();
        let sum_x2: f64 = (0..values.len()).map(|i| (i as f64).powi(2)).sum();
        
        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
        slope
    }
}
