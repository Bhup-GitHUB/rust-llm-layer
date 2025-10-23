use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexSimulation {
    pub table_name: String,
    pub column_names: Vec<String>,
    pub current_execution_time: u64,
    pub predicted_execution_time: u64,
    pub improvement_percent: f64,
    pub confidence_score: f64,
    pub storage_cost_mb: f64,
}

pub struct IndexUsageSimulator {
    base_performance_factor: f64,
    index_effectiveness_factor: f64,
}

impl IndexUsageSimulator {
    pub fn new() -> Self {
        Self {
            base_performance_factor: 0.1, // Index typically improves by 90%
            index_effectiveness_factor: 0.8, // 80% effectiveness for new indexes
        }
    }
    
    //yaha pe badme machine learning model bhi add karna ha
    pub fn simulate_index_impact(&self, 
        table_name: &str, 
        columns: &[String], 
        current_time: u64, 
        rows_scanned: u64) -> IndexSimulation {
        
        let predicted_time = self.calculate_predicted_time(current_time, rows_scanned, columns.len());
        let improvement = ((current_time as f64 - predicted_time as f64) / current_time as f64) * 100.0;
        let storage_cost = self.estimate_storage_cost(table_name, columns, rows_scanned);
        let confidence = self.calculate_confidence(columns.len(), rows_scanned);
        
        IndexSimulation {
            table_name: table_name.to_string(),
            column_names: columns.to_vec(),
            current_execution_time: current_time,
            predicted_execution_time: predicted_time,
            improvement_percent: improvement,
            confidence_score: confidence,
            storage_cost_mb: storage_cost,
        }
    }
    
    fn calculate_predicted_time(&self, current_time: u64, rows_scanned: u64, column_count: usize) -> u64 {
        let base_improvement = self.base_performance_factor;
        let column_factor = 1.0 - (column_count as f64 * 0.05); // More columns = better improvement
        let row_factor = if rows_scanned > 10000 {
            0.05 // Very good improvement for large scans
        } else if rows_scanned > 1000 {
            0.2 // Good improvement for medium scans
        } else {
            0.5 // Moderate improvement for small scans
        };
        
        let total_improvement = base_improvement * column_factor * row_factor;
        let predicted_time = (current_time as f64 * (1.0 - total_improvement)) as u64;
        
        predicted_time.max(1) // Minimum 1ms
    }
    
    fn estimate_storage_cost(&self, table_name: &str, columns: &[String], rows_scanned: u64) -> f64 {
        let base_size_per_row = 8.0; // 8 bytes per column
        let column_count = columns.len() as f64;
        let estimated_rows = rows_scanned as f64;
        
        let total_size_bytes = base_size_per_row * column_count * estimated_rows;
        total_size_bytes / (1024.0 * 1024.0) // Convert to MB
    }
    
    fn calculate_confidence(&self, column_count: usize, rows_scanned: u64) -> f64 {
        let column_confidence = if column_count == 1 {
            0.9 // Single column indexes are very reliable
        } else if column_count <= 3 {
            0.8 // Multi-column indexes are good
        } else {
            0.6 // Complex indexes are less predictable
        };
        
        let row_confidence = if rows_scanned > 10000 {
            0.9 // Large scans benefit more from indexes
        } else if rows_scanned > 1000 {
            0.8 // Medium scans benefit well
        } else {
            0.6 // Small scans have less predictable benefit
        };
        
        (column_confidence + row_confidence) / 2.0
    }
    
    pub fn simulate_multiple_indexes(&self, simulations: &[IndexSimulation]) -> Vec<IndexSimulation> {
        let mut results = simulations.to_vec();
        
        // Sort by improvement potential
        results.sort_by(|a, b| b.improvement_percent.partial_cmp(&a.improvement_percent).unwrap());
        
        results
    }
    
    pub fn get_roi_analysis(&self, simulation: &IndexSimulation) -> (f64, String) {
        let performance_gain = simulation.improvement_percent;
        let storage_cost = simulation.storage_cost_mb;
        
        let roi_score = if storage_cost > 0.0 {
            performance_gain / storage_cost
        } else {
            performance_gain
        };
        
        let recommendation = if roi_score > 50.0 {
            "Excellent ROI - highly recommended".to_string()
        } else if roi_score > 20.0 {
            "Good ROI - recommended".to_string()
        } else if roi_score > 10.0 {
            "Moderate ROI - consider carefully".to_string()
        } else {
            "Low ROI - not recommended".to_string()
        };
        
        (roi_score, recommendation)
    }
}
