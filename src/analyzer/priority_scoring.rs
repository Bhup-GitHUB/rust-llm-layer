use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityScore {
    pub index_name: String,
    pub total_score: f64,
    pub frequency_score: f64,
    pub performance_score: f64,
    pub cost_score: f64,
    pub complexity_score: f64,
    pub priority_level: String,
}

pub struct PriorityScoringAlgorithm {
    frequency_weight: f64,
    performance_weight: f64,
    cost_weight: f64,
    complexity_weight: f64,
}

impl PriorityScoringAlgorithm {
    pub fn new() -> Self {
        Self {
            frequency_weight: 0.3,
            performance_weight: 0.4,
            cost_weight: 0.2,
            complexity_weight: 0.1,
        }
    }
    
    //yaha pe badme machine learning weights bhi add karna ha
    pub fn calculate_priority_score(&self, 
        index_name: &str,
        frequency: u64,
        performance_impact: f64,
        maintenance_cost: f64,
        complexity: f64) -> PriorityScore {
        
        let frequency_score = self.score_frequency(frequency);
        let performance_score = self.score_performance(performance_impact);
        let cost_score = self.score_cost(maintenance_cost);
        let complexity_score = self.score_complexity(complexity);
        
        let total_score = (frequency_score * self.frequency_weight) +
                         (performance_score * self.performance_weight) +
                         (cost_score * self.cost_weight) +
                         (complexity_score * self.complexity_weight);
        
        let priority_level = self.determine_priority_level(total_score);
        
        PriorityScore {
            index_name: index_name.to_string(),
            total_score,
            frequency_score,
            performance_score,
            cost_score,
            complexity_score,
            priority_level,
        }
    }
    
    fn score_frequency(&self, frequency: u64) -> f64 {
        if frequency > 1000 {
            1.0 // Very high frequency
        } else if frequency > 500 {
            0.8 // High frequency
        } else if frequency > 100 {
            0.6 // Medium frequency
        } else if frequency > 10 {
            0.4 // Low frequency
        } else {
            0.2 // Very low frequency
        }
    }
    
    fn score_performance(&self, performance_impact: f64) -> f64 {
        if performance_impact > 50.0 {
            1.0 // Very high impact
        } else if performance_impact > 25.0 {
            0.8 // High impact
        } else if performance_impact > 10.0 {
            0.6 // Medium impact
        } else if performance_impact > 5.0 {
            0.4 // Low impact
        } else {
            0.2 // Very low impact
        }
    }
    
    fn score_cost(&self, maintenance_cost: f64) -> f64 {
        if maintenance_cost < 2.0 {
            1.0 // Very low cost
        } else if maintenance_cost < 5.0 {
            0.8 // Low cost
        } else if maintenance_cost < 10.0 {
            0.6 // Medium cost
        } else if maintenance_cost < 20.0 {
            0.4 // High cost
        } else {
            0.2 // Very high cost
        }
    }
    
    fn score_complexity(&self, complexity: f64) -> f64 {
        if complexity < 0.2 {
            1.0 // Very simple
        } else if complexity < 0.4 {
            0.8 // Simple
        } else if complexity < 0.6 {
            0.6 // Medium complexity
        } else if complexity < 0.8 {
            0.4 // Complex
        } else {
            0.2 // Very complex
        }
    }
    
    fn determine_priority_level(&self, total_score: f64) -> String {
        if total_score >= 0.8 {
            "Critical".to_string()
        } else if total_score >= 0.6 {
            "High".to_string()
        } else if total_score >= 0.4 {
            "Medium".to_string()
        } else if total_score >= 0.2 {
            "Low".to_string()
        } else {
            "Very Low".to_string()
        }
    }
    
    pub fn rank_indexes(&self, scores: &[PriorityScore]) -> Vec<&PriorityScore> {
        let mut ranked: Vec<&PriorityScore> = scores.iter().collect();
        ranked.sort_by(|a, b| b.total_score.partial_cmp(&a.total_score).unwrap());
        ranked
    }
    
    pub fn get_top_priorities(&self, scores: &[PriorityScore], limit: usize) -> Vec<&PriorityScore> {
        let ranked = self.rank_indexes(scores);
        ranked.into_iter().take(limit).collect()
    }
    
    //yaha pe badme dynamic weight adjustment bhi add karna ha
    pub fn adjust_weights(&mut self, 
        read_heavy: bool, 
        write_heavy: bool, 
        storage_constrained: bool) {
        
        if read_heavy {
            self.performance_weight = 0.5;
            self.frequency_weight = 0.3;
        }
        
        if write_heavy {
            self.cost_weight = 0.4;
            self.performance_weight = 0.3;
        }
        
        if storage_constrained {
            self.cost_weight = 0.5;
            self.complexity_weight = 0.2;
        }
    }
    
    pub fn calculate_roi_score(&self, 
        performance_gain: f64, 
        maintenance_cost: f64, 
        frequency: u64) -> f64 {
        
        if maintenance_cost == 0.0 {
            return performance_gain;
        }
        
        let frequency_factor = (frequency as f64 / 1000.0).min(1.0);
        (performance_gain * frequency_factor) / maintenance_cost
    }
}
