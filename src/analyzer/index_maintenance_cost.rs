use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceCost {
    pub index_name: String,
    pub table_name: String,
    pub write_overhead_percent: f64,
    pub insert_cost_ms: f64,
    pub update_cost_ms: f64,
    pub delete_cost_ms: f64,
    pub total_maintenance_cost: f64,
    pub recommendation: String,
}

pub struct IndexMaintenanceCostAnalyzer {
    base_write_time: f64,
    index_overhead_factor: f64,
}

impl IndexMaintenanceCostAnalyzer {
    pub fn new() -> Self {
        Self {
            base_write_time: 1.0, // Base write time in ms
            index_overhead_factor: 0.15, // 15% overhead per index
        }
    }
    
    //yaha pe badme advanced cost modeling bhi add karna ha
    pub fn calculate_maintenance_cost(&self, 
        index_name: &str, 
        table_name: &str, 
        column_count: usize,
        write_frequency: u64) -> MaintenanceCost {
        
        let overhead = self.calculate_index_overhead(column_count);
        let insert_cost = self.estimate_insert_cost(overhead, write_frequency);
        let update_cost = self.estimate_update_cost(overhead, write_frequency);
        let delete_cost = self.estimate_delete_cost(overhead, write_frequency);
        
        let total_cost = insert_cost + update_cost + delete_cost;
        let recommendation = self.generate_recommendation(total_cost, write_frequency);
        
        MaintenanceCost {
            index_name: index_name.to_string(),
            table_name: table_name.to_string(),
            write_overhead_percent: overhead * 100.0,
            insert_cost_ms: insert_cost,
            update_cost_ms: update_cost,
            delete_cost_ms: delete_cost,
            total_maintenance_cost: total_cost,
            recommendation,
        }
    }
    
    fn calculate_index_overhead(&self, column_count: usize) -> f64 {
        let base_overhead = self.index_overhead_factor;
        let column_multiplier = 1.0 + (column_count as f64 * 0.05);
        base_overhead * column_multiplier
    }
    
    fn estimate_insert_cost(&self, overhead: f64, write_frequency: u64) -> f64 {
        let base_cost = self.base_write_time;
        let frequency_factor = (write_frequency as f64 / 1000.0).min(2.0);
        base_cost * overhead * frequency_factor
    }
    
    fn estimate_update_cost(&self, overhead: f64, write_frequency: u64) -> f64 {
        let base_cost = self.base_write_time * 1.5; // Updates are more expensive
        let frequency_factor = (write_frequency as f64 / 1000.0).min(2.0);
        base_cost * overhead * frequency_factor
    }
    
    fn estimate_delete_cost(&self, overhead: f64, write_frequency: u64) -> f64 {
        let base_cost = self.base_write_time * 0.8; // Deletes are slightly cheaper
        let frequency_factor = (write_frequency as f64 / 1000.0).min(2.0);
        base_cost * overhead * frequency_factor
    }
    
    fn generate_recommendation(&self, total_cost: f64, write_frequency: u64) -> String {
        if total_cost > 10.0 && write_frequency > 1000 {
            "High maintenance cost - consider removing index".to_string()
        } else if total_cost > 5.0 {
            "Moderate maintenance cost - monitor performance".to_string()
        } else if total_cost < 2.0 {
            "Low maintenance cost - index is efficient".to_string()
        } else {
            "Acceptable maintenance cost".to_string()
        }
    }
    
    pub fn analyze_write_impact(&self, 
        current_write_time: f64, 
        index_count: usize) -> (f64, String) {
        
        let impact_percent = (index_count as f64 * self.index_overhead_factor) * 100.0;
        let new_write_time = current_write_time * (1.0 + (index_count as f64 * self.index_overhead_factor));
        
        let impact_level = if impact_percent > 50.0 {
            "High impact - significant write slowdown".to_string()
        } else if impact_percent > 25.0 {
            "Moderate impact - noticeable write slowdown".to_string()
        } else if impact_percent > 10.0 {
            "Low impact - minimal write slowdown".to_string()
        } else {
            "Negligible impact - no significant slowdown".to_string()
        };
        
        (new_write_time, impact_level)
    }
    
    //yaha pe badme batch operation analysis bhi add karna ha
    pub fn analyze_batch_operations(&self, 
        batch_size: u64, 
        index_count: usize) -> f64 {
        
        let base_batch_time = batch_size as f64 * 0.001; // 1ms per 1000 records
        let index_overhead = index_count as f64 * self.index_overhead_factor;
        
        base_batch_time * (1.0 + index_overhead)
    }
    
    pub fn get_optimal_index_count(&self, 
        read_frequency: u64, 
        write_frequency: u64) -> usize {
        
        let read_write_ratio = read_frequency as f64 / write_frequency as f64;
        
        if read_write_ratio > 10.0 {
            5 // High read ratio - more indexes beneficial
        } else if read_write_ratio > 5.0 {
            3 // Medium read ratio - moderate indexes
        } else if read_write_ratio > 2.0 {
            2 // Low read ratio - few indexes
        } else {
            1 // Very low read ratio - minimal indexes
        }
    }
}
