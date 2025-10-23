use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Query fingerprint for grouping similar queries - ye struct similar queries group karne ke liye hai
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFingerprint {
    pub fingerprint: String,
    pub query_count: u64,
    pub avg_execution_time: f64,
    pub sample_queries: Vec<String>,
    pub performance_trend: String, // "improving", "degrading", "stable"
}

/// Groups similar queries together - ye class similar queries group karta hai
pub struct QueryFingerprinter {
    fingerprints: HashMap<String, QueryFingerprint>,
}

impl QueryFingerprinter {
    pub fn new() -> Self {
        Self {
            fingerprints: HashMap::new(),
        }
    }
    
    /// Add query to fingerprinting - ye method query ko fingerprint mein add karta hai
    pub fn add_query(&mut self, query: &str, execution_time: u64) {
        let fingerprint = self.generate_fingerprint(query);
        
        let entry = self.fingerprints.entry(fingerprint.clone()).or_insert_with(|| {
            QueryFingerprint {
                fingerprint: fingerprint.clone(),
                query_count: 0,
                avg_execution_time: 0.0,
                sample_queries: Vec::new(),
                performance_trend: "stable".to_string(),
            }
        });
        
        // Update statistics - ye important hai performance tracking ke liye
        entry.query_count += 1;
        let total_time = entry.avg_execution_time * (entry.query_count - 1) as f64 + execution_time as f64;
        entry.avg_execution_time = total_time / entry.query_count as f64;
        
        // Keep sample queries (max 5)
        if entry.sample_queries.len() < 5 {
            entry.sample_queries.push(query.to_string());
        }
        
        // Update performance trend - ye trend analysis ke liye hai
        self.update_performance_trend(entry);
    }
    
    fn generate_fingerprint(&self, query: &str) -> String {
        let query_upper = query.to_uppercase();
        let mut fingerprint = String::new();
        
        // Extract basic structure - ye query structure identify karta hai
        if query_upper.contains("SELECT") {
            fingerprint.push_str("SELECT_");
        }
        if query_upper.contains("FROM") {
            fingerprint.push_str("FROM_");
        }
        if query_upper.contains("WHERE") {
            fingerprint.push_str("WHERE_");
        }
        if query_upper.contains("JOIN") {
            fingerprint.push_str("JOIN_");
        }
        if query_upper.contains("ORDER BY") {
            fingerprint.push_str("ORDER_");
        }
        if query_upper.contains("GROUP BY") {
            fingerprint.push_str("GROUP_");
        }
        
        // Add table count - ye table count add karta hai
        let table_count = query_upper.matches("FROM").count() + query_upper.matches("JOIN").count();
        fingerprint.push_str(&format!("TABLES_{}", table_count));
        
        // Add condition count - ye condition count add karta hai
        let condition_count = query_upper.matches("AND").count() + query_upper.matches("OR").count();
        fingerprint.push_str(&format!("_CONDITIONS_{}", condition_count));
        
        fingerprint
    }
    
    fn update_performance_trend(&self, fingerprint: &mut QueryFingerprint) {
        // Simple trend analysis - ye basic trend analysis hai
        if fingerprint.query_count >= 3 {
            let recent_avg = fingerprint.avg_execution_time;
            if recent_avg > 200.0 {
                fingerprint.performance_trend = "degrading".to_string();
            } else if recent_avg < 50.0 {
                fingerprint.performance_trend = "improving".to_string();
            } else {
                fingerprint.performance_trend = "stable".to_string();
            }
        }
    }
    
    /// Get similar query groups - ye method similar query groups return karta hai
    pub fn get_similar_groups(&self, min_count: u64) -> Vec<&QueryFingerprint> {
        self.fingerprints
            .values()
            .filter(|fp| fp.query_count >= min_count)
            .collect()
    }
    
    /// Get performance insights - ye method performance insights deta hai
    pub fn get_performance_insights(&self) -> Vec<String> {
        let mut insights = Vec::new();
        
        let total_fingerprints = self.fingerprints.len();
        let high_frequency = self.fingerprints.values()
            .filter(|fp| fp.query_count >= 10)
            .count();
        
        insights.push(format!(
            "Total unique query patterns: {}, High frequency patterns: {}",
            total_fingerprints, high_frequency
        ));
        
        // Find degrading patterns - ye degrading patterns find karta hai
        let degrading_patterns: Vec<_> = self.fingerprints.values()
            .filter(|fp| fp.performance_trend == "degrading")
            .collect();
            
        if !degrading_patterns.is_empty() {
            insights.push(format!(
                "Warning: {} query patterns showing performance degradation",
                degrading_patterns.len()
            ));
        }
        
        // Find improving patterns
        let improving_patterns: Vec<_> = self.fingerprints.values()
            .filter(|fp| fp.performance_trend == "improving")
            .collect();
            
        if !improving_patterns.is_empty() {
            insights.push(format!(
                "Good news: {} query patterns showing performance improvement",
                improving_patterns.len()
            ));
        }
        
        insights
    }
    
    /// Get optimization candidates - ye method optimization candidates suggest karta hai
    pub fn get_optimization_candidates(&self) -> Vec<&QueryFingerprint> {
        self.fingerprints
            .values()
            .filter(|fp| fp.query_count >= 5 && fp.avg_execution_time > 100.0)
            .collect()
    }
}
