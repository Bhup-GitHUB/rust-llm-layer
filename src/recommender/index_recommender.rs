use crate::analyzer::QueryPattern;

/// Index recommendation for database optimization - database optimization ke liye index recommend karta hai
#[derive(Debug, Clone)]
pub struct IndexRecommendation {
    pub table: String,
    pub column: String,
    pub index_type: IndexType,
    pub priority: u32,
    pub estimated_improvement_percent: f64,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub enum IndexType {
    BTree,
    Hash,
}

/// Recommends database indexes based on query patterns - query patterns ke basis pe indexes suggest karta hai
pub struct IndexRecommender {
    slowness_threshold: f64,
    frequency_threshold: u64,
}

impl IndexRecommender {
    pub fn new(slowness_threshold: f64, frequency_threshold: u64) -> Self {
        Self {
            slowness_threshold,
            frequency_threshold,
        }
    }

    pub fn recommend(&self, patterns: &[QueryPattern]) -> Vec<IndexRecommendation> {
        let mut recommendations = Vec::new();

        for pattern in patterns {
            if pattern.slowness_score > self.slowness_threshold
                || pattern.frequency > self.frequency_threshold
            {
                for table in &pattern.tables {
                    let improvement = self.calculate_improvement(pattern);
                    let priority = self.calculate_priority(pattern);

                    let recommendation = IndexRecommendation {
                        table: table.clone(),
                        column: "id".to_string(),
                        index_type: self.suggest_index_type(pattern),
                        priority,
                        estimated_improvement_percent: improvement,
                        reason: self.generate_reason(pattern),
                    };

                    recommendations.push(recommendation);
                }
            }
        }

        recommendations.sort_by(|a, b| b.priority.cmp(&a.priority));
        recommendations
    }

    fn calculate_improvement(&self, pattern: &QueryPattern) -> f64 {
        let base_improvement = 40.0;
        let frequency_bonus = (pattern.frequency as f64 / 100.0).min(30.0);
        let slowness_bonus = (pattern.slowness_score / 10000.0).min(20.0);
        base_improvement + frequency_bonus + slowness_bonus
    }

    fn calculate_priority(&self, pattern: &QueryPattern) -> u32 {
        let base = if pattern.slowness_score > 10000.0 {
            100
        } else {
            50
        };

        let freq_bonus = (pattern.frequency / 10).min(50);
        base + freq_bonus as u32
    }

    fn suggest_index_type(&self, pattern: &QueryPattern) -> IndexType {
        if pattern.query_type == "SELECT" && pattern.frequency > 100 {
            IndexType::Hash
        } else {
            IndexType::BTree
        }
    }

    fn generate_reason(&self, pattern: &QueryPattern) -> String {
        format!(
            "Query type: {}, Frequency: {}, Avg time: {:.2}ms",
            pattern.query_type, pattern.frequency, pattern.avg_execution_time_ms
        )
    }
}