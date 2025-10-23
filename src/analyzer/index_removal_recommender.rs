use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexUsageStats {
    pub index_name: String,
    pub table_name: String,
    pub usage_count: u64,
    pub last_used: u64,
    pub query_benefit: f64,
    pub maintenance_cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemovalRecommendation {
    pub index_name: String,
    pub table_name: String,
    pub removal_reason: String,
    pub confidence_score: f64,
    pub estimated_savings: f64,
    pub risk_level: String,
    pub sql_statement: String,
}

pub struct IndexRemovalRecommender {
    usage_threshold: u64,
    time_threshold: u64,
    benefit_threshold: f64,
}

impl IndexRemovalRecommender {
    pub fn new() -> Self {
        Self {
            usage_threshold: 10, // Minimum usage count
            time_threshold: 86400 * 30, // 30 days in seconds
            benefit_threshold: 0.1, // Minimum benefit ratio
        }
    }
    
    //yaha pe badme advanced usage analysis bhi add karna ha
    pub fn analyze_index_usage(&self, usage_stats: &[IndexUsageStats]) -> Vec<RemovalRecommendation> {
        let mut recommendations = Vec::new();
        
        for stats in usage_stats {
            if let Some(recommendation) = self.evaluate_index_for_removal(stats) {
                recommendations.push(recommendation);
            }
        }
        
        // Sort by confidence score (highest first)
        recommendations.sort_by(|a, b| b.confidence_score.partial_cmp(&a.confidence_score).unwrap());
        recommendations
    }
    
    fn evaluate_index_for_removal(&self, stats: &IndexUsageStats) -> Option<RemovalRecommendation> {
        let mut reasons = Vec::new();
        let mut confidence = 0.0;
        
        // Check usage frequency
        if stats.usage_count < self.usage_threshold {
            reasons.push("Low usage frequency".to_string());
            confidence += 0.3;
        }
        
        // Check last usage time
        if stats.last_used < self.time_threshold {
            reasons.push("Not used recently".to_string());
            confidence += 0.2;
        }
        
        // Check benefit vs cost ratio
        if stats.query_benefit < stats.maintenance_cost * self.benefit_threshold {
            reasons.push("Low benefit compared to maintenance cost".to_string());
            confidence += 0.4;
        }
        
        // Check for zero usage
        if stats.usage_count == 0 {
            reasons.push("Never used".to_string());
            confidence += 0.5;
        }
        
        if confidence > 0.5 {
            let risk_level = self.determine_risk_level(confidence, stats.usage_count);
            let estimated_savings = self.calculate_savings(stats);
            let sql_statement = self.generate_drop_sql(stats);
            
            Some(RemovalRecommendation {
                index_name: stats.index_name.clone(),
                table_name: stats.table_name.clone(),
                removal_reason: reasons.join(", "),
                confidence_score: confidence,
                estimated_savings,
                risk_level,
                sql_statement,
            })
        } else {
            None
        }
    }
    
    fn determine_risk_level(&self, confidence: f64, usage_count: u64) -> String {
        if confidence > 0.8 && usage_count == 0 {
            "Very Low".to_string()
        } else if confidence > 0.7 && usage_count < 5 {
            "Low".to_string()
        } else if confidence > 0.6 {
            "Medium".to_string()
        } else {
            "High".to_string()
        }
    }
    
    fn calculate_savings(&self, stats: &IndexUsageStats) -> f64 {
        // Estimate storage savings and maintenance cost reduction
        let storage_savings = 50.0; // MB (estimated)
        let maintenance_savings = stats.maintenance_cost * 100.0; // Convert to percentage
        
        storage_savings + maintenance_savings
    }
    
    fn generate_drop_sql(&self, stats: &IndexUsageStats) -> String {
        format!("DROP INDEX {} ON {}", stats.index_name, stats.table_name)
    }
    
    pub fn find_redundant_indexes(&self, indexes: &[IndexUsageStats]) -> Vec<RemovalRecommendation> {
        let mut redundant = Vec::new();
        let mut grouped: HashMap<String, Vec<&IndexUsageStats>> = HashMap::new();
        
        // Group indexes by table
        for index in indexes {
            grouped.entry(index.table_name.clone()).or_insert_with(Vec::new).push(index);
        }
        
        // Find redundant indexes within each table
        for (_, table_indexes) in grouped {
            if table_indexes.len() > 1 {
                let mut sorted_indexes = table_indexes.clone();
                sorted_indexes.sort_by(|a, b| a.usage_count.cmp(&b.usage_count));
                
                // Mark lower usage indexes as redundant
                for (i, index) in sorted_indexes.iter().enumerate() {
                    if i < sorted_indexes.len() - 1 { // Not the most used index
                        redundant.push(RemovalRecommendation {
                            index_name: index.index_name.clone(),
                            table_name: index.table_name.clone(),
                            removal_reason: "Redundant - lower usage than other indexes".to_string(),
                            confidence_score: 0.7,
                            estimated_savings: 30.0,
                            risk_level: "Medium".to_string(),
                            sql_statement: self.generate_drop_sql(index),
                        });
                    }
                }
            }
        }
        
        redundant
    }
    
    //yaha pe badme index dependency analysis bhi add karna ha
    pub fn get_safe_removal_candidates(&self, recommendations: &[RemovalRecommendation]) -> Vec<&RemovalRecommendation> {
        recommendations
            .iter()
            .filter(|rec| rec.risk_level == "Very Low" || rec.risk_level == "Low")
            .collect()
    }
    
    pub fn get_removal_summary(&self, recommendations: &[RemovalRecommendation]) -> (usize, f64, usize) {
        let total_recommendations = recommendations.len();
        let total_savings: f64 = recommendations.iter().map(|r| r.estimated_savings).sum();
        let safe_removals = self.get_safe_removal_candidates(recommendations).len();
        
        (total_recommendations, total_savings, safe_removals)
    }
}
