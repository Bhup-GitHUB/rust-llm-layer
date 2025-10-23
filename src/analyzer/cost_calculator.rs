use serde::{Deserialize, Serialize};

/// Query cost calculation - ye struct query cost calculate karta hai
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryCost {
    pub base_cost: f64,
    pub row_scan_cost: f64,
    pub join_cost: f64,
    pub sort_cost: f64,
    pub total_cost: f64,
    pub cost_category: String, // "low", "medium", "high"
}

/// Calculates query execution cost - ye class query cost calculate karta hai
pub struct CostCalculator {
    base_row_cost: f64,
    join_multiplier: f64,
    sort_multiplier: f64,
}

impl CostCalculator {
    pub fn new() -> Self {
        Self {
            base_row_cost: 0.001, // Base cost per row
            join_multiplier: 1.5,  // Join complexity multiplier
            sort_multiplier: 2.0, // Sort complexity multiplier
        }
    }
    
    /// Calculate cost for a query - ye method query ka cost calculate karta hai
    pub fn calculate_cost(&self, 
        rows_scanned: u64, 
        execution_time: u64, 
        join_count: usize, 
        has_order_by: bool,
        has_group_by: bool) -> QueryCost {
        
        // Base cost calculation - ye base cost calculate karta hai
        let base_cost = execution_time as f64;
        
        // Row scan cost - ye row scan cost calculate karta hai
        let row_scan_cost = rows_scanned as f64 * self.base_row_cost;
        
        // Join cost calculation - ye join cost calculate karta hai
        let join_cost = if join_count > 0 {
            row_scan_cost * self.join_multiplier * join_count as f64
        } else {
            0.0
        };
        
        // Sort cost calculation - ye sort cost calculate karta hai
        let sort_cost = if has_order_by || has_group_by {
            row_scan_cost * self.sort_multiplier
        } else {
            0.0
        };
        
        let total_cost = base_cost + row_scan_cost + join_cost + sort_cost;
        
        // Determine cost category - ye cost category determine karta hai
        let cost_category = if total_cost < 10.0 {
            "low".to_string()
        } else if total_cost < 100.0 {
            "medium".to_string()
        } else {
            "high".to_string()
        };
        
        QueryCost {
            base_cost,
            row_scan_cost,
            join_cost,
            sort_cost,
            total_cost,
            cost_category,
        }
    }
    
    /// Calculate cost from parsed query - ye method parsed query se cost calculate karta hai
    pub fn calculate_from_parsed(&self, 
        parsed_query: &crate::analyzer::query_parser::ParsedQuery,
        execution_time: u64,
        rows_scanned: u64) -> QueryCost {
        
        let join_count = parsed_query.join_conditions.len();
        let has_order_by = !parsed_query.order_by_columns.is_empty();
        let has_group_by = false; // Would need to detect GROUP BY in parser
        
        self.calculate_cost(
            rows_scanned,
            execution_time,
            join_count,
            has_order_by,
            has_group_by
        )
    }
    
    /// Get cost optimization suggestions - ye method cost optimization suggestions deta hai
    pub fn get_optimization_suggestions(&self, cost: &QueryCost) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // High row scan cost suggestions - ye high row scan cost ke liye suggestions hai
        if cost.row_scan_cost > 50.0 {
            suggestions.push("High row scan cost detected - consider adding indexes on WHERE clause columns".to_string());
        }
        
        // High join cost suggestions - ye high join cost ke liye suggestions hai
        if cost.join_cost > 100.0 {
            suggestions.push("High join cost detected - review join conditions and consider denormalization".to_string());
        }
        
        // High sort cost suggestions - ye high sort cost ke liye suggestions hai
        if cost.sort_cost > 50.0 {
            suggestions.push("High sort cost detected - consider pre-sorted indexes or limit result set".to_string());
        }
        
        // Overall cost suggestions - ye overall cost ke liye suggestions hai
        if cost.total_cost > 200.0 {
            suggestions.push("Very high query cost - consider query rewriting or caching strategy".to_string());
        }
        
        suggestions
    }
    
    /// Compare costs for optimization - ye method costs compare karta hai optimization ke liye
    pub fn compare_costs(&self, original_cost: &QueryCost, optimized_cost: &QueryCost) -> (f64, String) {
        let improvement_percent = ((original_cost.total_cost - optimized_cost.total_cost) / original_cost.total_cost) * 100.0;
        
        let improvement_level = if improvement_percent > 50.0 {
            "Excellent".to_string()
        } else if improvement_percent > 20.0 {
            "Good".to_string()
        } else if improvement_percent > 0.0 {
            "Moderate".to_string()
        } else {
            "No improvement".to_string()
        };
        
        (improvement_percent, improvement_level)
    }
    
    /// Get cost summary - ye method cost summary deta hai
    pub fn get_cost_summary(&self, costs: &[QueryCost]) -> (f64, f64, usize, usize) {
        if costs.is_empty() {
            return (0.0, 0.0, 0, 0);
        }
        
        let total_cost: f64 = costs.iter().map(|c| c.total_cost).sum();
        let avg_cost = total_cost / costs.len() as f64;
        let high_cost_count = costs.iter().filter(|c| c.cost_category == "high").count();
        let low_cost_count = costs.iter().filter(|c| c.cost_category == "low").count();
        
        (total_cost, avg_cost, high_cost_count, low_cost_count)
    }
}
