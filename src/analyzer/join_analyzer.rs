use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Table join analysis - ye struct table joins analyze karta hai
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinPattern {
    pub table1: String,
    pub table2: String,
    pub join_count: u64,
    pub avg_execution_time: f64,
    pub join_type: String, // INNER, LEFT, RIGHT, etc.
    pub performance_score: f64,
}

/// Analyzes table join patterns - ye class table joins analyze karta hai
pub struct JoinAnalyzer {
    join_stats: HashMap<String, JoinPattern>,
}

impl JoinAnalyzer {
    pub fn new() -> Self {
        Self {
            join_stats: HashMap::new(),
        }
    }
    
    /// Analyze join from parsed query - ye method join analyze karta hai
    pub fn analyze_join(&mut self, parsed_query: &crate::analyzer::query_parser::ParsedQuery, execution_time: u64) {
        if parsed_query.join_conditions.is_empty() {
            return;
        }
        
        // Extract table pairs from joins - ye important hai join analysis ke liye
        for join_condition in &parsed_query.join_conditions {
            if let Some((table1, table2)) = self.extract_table_pair(join_condition) {
                let key = self.create_join_key(&table1, &table2);
                
                let entry = self.join_stats.entry(key).or_insert_with(|| {
                    JoinPattern {
                        table1: table1.clone(),
                        table2: table2.clone(),
                        join_count: 0,
                        avg_execution_time: 0.0,
                        join_type: self.detect_join_type(join_condition),
                        performance_score: 0.0,
                    }
                });
                
                entry.join_count += 1;
                let total_time = entry.avg_execution_time * (entry.join_count - 1) as f64 + execution_time as f64;
                entry.avg_execution_time = total_time / entry.join_count as f64;
                
                // Calculate performance score - ye performance score calculate karta hai
                entry.performance_score = self.calculate_performance_score(entry);
            }
        }
    }
    
    fn extract_table_pair(&self, join_condition: &str) -> Option<(String, String)> {
        // Simple table pair extraction - ye basic extraction hai
        let parts: Vec<&str> = join_condition.split_whitespace().collect();
        if parts.len() >= 2 {
            let table1 = parts[0].to_string();
            let table2 = if parts.len() > 2 { parts[2].to_string() } else { parts[1].to_string() };
            Some((table1, table2))
        } else {
            None
        }
    }
    
    fn create_join_key(&self, table1: &str, table2: &str) -> String {
        // Create consistent key for table pairs - ye consistent key banata hai
        if table1 < table2 {
            format!("{}_JOIN_{}", table1, table2)
        } else {
            format!("{}_JOIN_{}", table2, table1)
        }
    }
    
    fn detect_join_type(&self, join_condition: &str) -> String {
        let condition_upper = join_condition.to_uppercase();
        if condition_upper.contains("LEFT JOIN") {
            "LEFT".to_string()
        } else if condition_upper.contains("RIGHT JOIN") {
            "RIGHT".to_string()
        } else if condition_upper.contains("INNER JOIN") {
            "INNER".to_string()
        } else {
            "INNER".to_string() // Default
        }
    }
    
    fn calculate_performance_score(&self, join_pattern: &JoinPattern) -> f64 {
        // Performance score based on frequency and execution time - ye performance score calculate karta hai
        let frequency_score = (join_pattern.join_count as f64 / 100.0).min(1.0);
        let time_score = (1000.0 / join_pattern.avg_execution_time).min(1.0);
        (frequency_score + time_score) / 2.0
    }
    
    /// Get most frequent joins - ye method sabse zyada frequent joins deta hai
    pub fn get_frequent_joins(&self, limit: usize) -> Vec<&JoinPattern> {
        let mut joins: Vec<&JoinPattern> = self.join_stats.values().collect();
        joins.sort_by(|a, b| b.join_count.cmp(&a.join_count));
        joins.into_iter().take(limit).collect()
    }
    
    /// Get slow joins - ye method slow joins identify karta hai
    pub fn get_slow_joins(&self, threshold_ms: f64) -> Vec<&JoinPattern> {
        self.join_stats
            .values()
            .filter(|join| join.avg_execution_time > threshold_ms)
            .collect()
    }
    
    /// Get join recommendations - ye method join recommendations deta hai
    pub fn get_join_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // Check for frequent slow joins - ye frequent slow joins check karta hai
        let slow_joins = self.get_slow_joins(200.0);
        let frequent_slow_joins: Vec<_> = slow_joins.iter()
            .filter(|join| join.join_count >= 5)
            .collect();
            
        if !frequent_slow_joins.is_empty() {
            recommendations.push(format!(
                "Found {} frequent slow joins - consider adding indexes or optimizing join conditions",
                frequent_slow_joins.len()
            ));
        }
        
        // Check for performance trends - ye performance trends check karta hai
        let high_performance_joins: Vec<_> = self.join_stats.values()
            .filter(|join| join.performance_score > 0.8)
            .collect();
            
        if !high_performance_joins.is_empty() {
            recommendations.push(format!(
                "Good performance detected in {} join patterns - maintain current optimization",
                high_performance_joins.len()
            ));
        }
        
        recommendations
    }
    
    /// Get join statistics summary - ye method join statistics summary deta hai
    pub fn get_join_summary(&self) -> (usize, f64, usize) {
        let total_joins = self.join_stats.len();
        let avg_performance = if total_joins > 0 {
            self.join_stats.values().map(|j| j.performance_score).sum::<f64>() / total_joins as f64
        } else {
            0.0
        };
        let slow_joins = self.get_slow_joins(100.0).len();
        
        (total_joins, avg_performance, slow_joins)
    }
}
