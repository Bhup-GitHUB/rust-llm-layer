use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadlockInfo {
    pub deadlock_id: String,
    pub timestamp: u64,
    pub involved_queries: Vec<String>,
    pub locked_tables: Vec<String>,
    pub wait_time: u64,
    pub resolution_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadlockPrevention {
    pub query_pattern: String,
    pub risk_level: String,
    pub prevention_strategy: String,
    pub recommended_changes: Vec<String>,
}

pub struct DeadlockDetector {
    deadlock_history: Vec<DeadlockInfo>,
    query_patterns: HashMap<String, u64>,
    lock_sequences: HashMap<String, Vec<String>>,
}

impl DeadlockDetector {
    pub fn new() -> Self {
        Self {
            deadlock_history: Vec::new(),
            query_patterns: HashMap::new(),
            lock_sequences: HashMap::new(),
        }
    }
    
    //yaha pe badme advanced deadlock analysis bhi add karna ha
    pub fn record_deadlock(&mut self, deadlock: DeadlockInfo) {
        self.deadlock_history.push(deadlock.clone());
        
        // Track query patterns involved in deadlocks
        for query in &deadlock.involved_queries {
            *self.query_patterns.entry(query.clone()).or_insert(0) += 1;
        }
        
        // Track lock sequences
        let sequence_key = deadlock.locked_tables.join("->");
        self.lock_sequences.entry(sequence_key).or_insert_with(Vec::new)
            .extend(deadlock.involved_queries.clone());
    }
    
    pub fn analyze_deadlock_patterns(&self) -> Vec<DeadlockPrevention> {
        let mut preventions = Vec::new();
        
        // Find frequently deadlocking queries
        for (query, count) in &self.query_patterns {
            if *count > 1 {
                preventions.push(DeadlockPrevention {
                    query_pattern: query.clone(),
                    risk_level: self.determine_risk_level(*count),
                    prevention_strategy: self.suggest_prevention_strategy(query),
                    recommended_changes: self.get_recommended_changes(query),
                });
            }
        }
        
        // Analyze lock sequences
        for (sequence, queries) in &self.lock_sequences {
            if queries.len() > 1 {
                preventions.push(DeadlockPrevention {
                    query_pattern: format!("Lock sequence: {}", sequence),
                    risk_level: "High".to_string(),
                    prevention_strategy: "Standardize lock order".to_string(),
                    recommended_changes: vec![
                        "Always acquire locks in the same order".to_string(),
                        "Use shorter transactions".to_string(),
                        "Consider lock timeouts".to_string(),
                    ],
                });
            }
        }
        
        preventions
    }
    
    fn determine_risk_level(&self, deadlock_count: u64) -> String {
        if deadlock_count > 5 {
            "Critical".to_string()
        } else if deadlock_count > 3 {
            "High".to_string()
        } else if deadlock_count > 1 {
            "Medium".to_string()
        } else {
            "Low".to_string()
        }
    }
    
    fn suggest_prevention_strategy(&self, query: &str) -> String {
        let query_lower = query.to_lowercase();
        
        if query_lower.contains("update") && query_lower.contains("where") {
            "Use row-level locking and shorter transactions".to_string()
        } else if query_lower.contains("delete") {
            "Batch delete operations and use proper indexing".to_string()
        } else if query_lower.contains("insert") {
            "Use bulk insert operations and avoid long transactions".to_string()
        } else {
            "Optimize query and reduce transaction scope".to_string()
        }
    }
    
    fn get_recommended_changes(&self, query: &str) -> Vec<String> {
        let mut changes = Vec::new();
        
        if query.contains("SELECT *") {
            changes.push("Replace SELECT * with specific columns".to_string());
        }
        
        if query.contains("UPDATE") && !query.contains("WHERE") {
            changes.push("Add WHERE clause to UPDATE statement".to_string());
        }
        
        if query.len() > 1000 {
            changes.push("Break down complex query into smaller parts".to_string());
        }
        
        changes.push("Add proper indexes to reduce lock time".to_string());
        changes.push("Use READ COMMITTED isolation level".to_string());
        
        changes
    }
    
    pub fn get_deadlock_statistics(&self) -> (usize, f64, String) {
        let total_deadlocks = self.deadlock_history.len();
        let avg_resolution_time = if total_deadlocks > 0 {
            self.deadlock_history.iter()
                .map(|d| d.resolution_time)
                .sum::<u64>() as f64 / total_deadlocks as f64
        } else {
            0.0
        };
        
        let trend = if total_deadlocks > 10 {
            let recent_deadlocks = self.deadlock_history.iter()
                .rev()
                .take(5)
                .count();
            let older_deadlocks = self.deadlock_history.iter()
                .rev()
                .skip(5)
                .take(5)
                .count();
            
            if recent_deadlocks > older_deadlocks {
                "increasing".to_string()
            } else if recent_deadlocks < older_deadlocks {
                "decreasing".to_string()
            } else {
                "stable".to_string()
            }
        } else {
            "insufficient_data".to_string()
        };
        
        (total_deadlocks, avg_resolution_time, trend)
    }
    
    //yaha pe badme predictive deadlock detection bhi add karna ha
    pub fn predict_deadlock_risk(&self, new_query: &str, current_locks: &[String]) -> f64 {
        let mut risk_score = 0.0;
        
        // Check if query pattern has caused deadlocks before
        if let Some(&count) = self.query_patterns.get(new_query) {
            risk_score += count as f64 * 0.3;
        }
        
        // Check for potential lock conflicts
        for lock in current_locks {
            if new_query.contains(lock) {
                risk_score += 0.5;
            }
        }
        
        // Check query complexity
        if new_query.len() > 500 {
            risk_score += 0.2;
        }
        
        if new_query.contains("UPDATE") || new_query.contains("DELETE") {
            risk_score += 0.3;
        }
        
        risk_score.min(1.0) // Cap at 1.0
    }
    
    pub fn get_optimization_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if self.deadlock_history.len() > 5 {
            recommendations.push("Consider implementing deadlock detection timeout".to_string());
            recommendations.push("Review and optimize transaction boundaries".to_string());
        }
        
        let high_risk_queries: Vec<_> = self.query_patterns.iter()
            .filter(|(_, &count)| count > 2)
            .collect();
            
        if !high_risk_queries.is_empty() {
            recommendations.push("High-risk queries identified - consider query optimization".to_string());
        }
        
        recommendations.push("Implement proper indexing strategy".to_string());
        recommendations.push("Use appropriate isolation levels".to_string());
        recommendations.push("Consider connection pooling optimization".to_string());
        
        recommendations
    }
}
