use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialIndexRecommendation {
    pub table_name: String,
    pub column_names: Vec<String>,
    pub filter_condition: String,
    pub selectivity_ratio: f64,
    pub estimated_storage_savings: f64,
    pub performance_impact: f64,
    pub sql_statement: String,
}

pub struct PartialIndexRecommender {
    condition_patterns: HashMap<String, f64>,
}

impl PartialIndexRecommender {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        
        // Common high-selectivity conditions
        patterns.insert("active = true".to_string(), 0.1);
        patterns.insert("status = 'active'".to_string(), 0.15);
        patterns.insert("deleted = false".to_string(), 0.05);
        patterns.insert("created_at > ".to_string(), 0.2);
        patterns.insert("is_verified = true".to_string(), 0.3);
        
        Self {
            condition_patterns: patterns,
        }
    }
    
    //yaha pe badme advanced pattern matching bhi add karna ha
    pub fn analyze_query_for_partial_index(&self, 
        query: &str, 
        table_name: &str, 
        columns: &[String]) -> Option<PartialIndexRecommendation> {
        
        let conditions = self.extract_where_conditions(query);
        let best_condition = self.find_best_partial_condition(&conditions);
        
        if let Some(condition) = best_condition {
            let selectivity = self.calculate_selectivity(&condition);
            let storage_savings = self.estimate_storage_savings(selectivity);
            let performance_impact = self.estimate_performance_impact(selectivity);
            let sql = self.generate_partial_index_sql(table_name, columns, &condition);
            
            Some(PartialIndexRecommendation {
                table_name: table_name.to_string(),
                column_names: columns.to_vec(),
                filter_condition: condition,
                selectivity_ratio: selectivity,
                estimated_storage_savings: storage_savings,
                performance_impact: performance_impact,
                sql_statement: sql,
            })
        } else {
            None
        }
    }
    
    fn extract_where_conditions(&self, query: &str) -> Vec<String> {
        let query_upper = query.to_uppercase();
        let mut conditions = Vec::new();
        
        if let Some(where_start) = query_upper.find("WHERE") {
            let where_clause = &query[where_start + 5..];
            let conditions_str = where_clause.split_whitespace().collect::<Vec<&str>>().join(" ");
            
            // Split by AND/OR but keep the logic
            let parts: Vec<&str> = conditions_str.split(" AND ").collect();
            for part in parts {
                let or_parts: Vec<&str> = part.split(" OR ").collect();
                for or_part in or_parts {
                    conditions.push(or_part.trim().to_string());
                }
            }
        }
        
        conditions
    }
    
    fn find_best_partial_condition(&self, conditions: &[String]) -> Option<String> {
        let mut best_condition = None;
        let mut best_score = 0.0;
        
        for condition in conditions {
            let score = self.score_condition(condition);
            if score > best_score {
                best_score = score;
                best_condition = Some(condition.clone());
            }
        }
        
        if best_score > 0.3 { // Minimum threshold for partial index
            best_condition
        } else {
            None
        }
    }
    
    fn score_condition(&self, condition: &str) -> f64 {
        let condition_lower = condition.to_lowercase();
        
        // Check against known patterns
        for (pattern, score) in &self.condition_patterns {
            if condition_lower.contains(pattern) {
                return *score;
            }
        }
        
        // Generic scoring based on condition type
        if condition_lower.contains("= true") || condition_lower.contains("= false") {
            0.2
        } else if condition_lower.contains(">") || condition_lower.contains("<") {
            0.15
        } else if condition_lower.contains("in (") {
            0.1
        } else {
            0.05
        }
    }
    
    fn calculate_selectivity(&self, condition: &str) -> f64 {
        // Simple selectivity estimation
        if condition.contains("= true") || condition.contains("= false") {
            0.1 // Boolean conditions are usually highly selective
        } else if condition.contains(">") {
            0.3 // Range conditions
        } else if condition.contains("in (") {
            0.2 // IN clauses
        } else {
            0.5 // Default moderate selectivity
        }
    }
    
    fn estimate_storage_savings(&self, selectivity: f64) -> f64 {
        // Storage savings = (1 - selectivity) * estimated_index_size
        let base_index_size = 100.0; // MB
        (1.0 - selectivity) * base_index_size
    }
    
    fn estimate_performance_impact(&self, selectivity: f64) -> f64 {
        // Lower selectivity = better performance impact
        (1.0 - selectivity) * 100.0
    }
    
    fn generate_partial_index_sql(&self, table_name: &str, columns: &[String], condition: &str) -> String {
        let column_list = columns.join(", ");
        format!(
            "CREATE INDEX idx_{}_partial ON {} ({}) WHERE {}",
            table_name,
            table_name,
            column_list,
            condition
        )
    }
    
    pub fn get_high_selectivity_conditions(&self) -> Vec<String> {
        self.condition_patterns
            .iter()
            .filter(|(_, score)| **score < 0.2) // High selectivity conditions
            .map(|(condition, _)| condition.clone())
            .collect()
    }
    
    //yaha pe badme condition optimization bhi add karna ha
    pub fn optimize_condition(&self, condition: &str) -> String {
        let mut optimized = condition.to_string();
        
        // Simple optimizations
        optimized = optimized.replace(" = true", "");
        optimized = optimized.replace(" = false", " = false");
        optimized = optimized.replace("  ", " "); // Remove double spaces
        
        optimized
    }
}
