use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryPlan {
    pub plan_id: String,
    pub query_text: String,
    pub execution_time: u64,
    pub cost_estimate: f64,
    pub operations: Vec<PlanOperation>,
    pub optimization_suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanOperation {
    pub operation_type: String, // "Seq Scan", "Index Scan", "Hash Join", etc.
    pub table_name: String,
    pub cost: f64,
    pub rows: u64,
    pub width: u64,
    pub is_expensive: bool,
}

pub struct QueryPlanAnalyzer {
    cost_threshold: f64,
    time_threshold: u64,
}

impl QueryPlanAnalyzer {
    pub fn new() -> Self {
        Self {
            cost_threshold: 1000.0,
            time_threshold: 100, // 100ms
        }
    }
    
    //yaha pe badme advanced plan parsing bhi add karna ha
    pub fn analyze_plan(&self, plan: &QueryPlan) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // Analyze expensive operations
        for operation in &plan.operations {
            if operation.is_expensive {
                suggestions.push(self.suggest_optimization(operation));
            }
        }
        
        // Check for sequential scans
        let seq_scans: Vec<_> = plan.operations.iter()
            .filter(|op| op.operation_type == "Seq Scan")
            .collect();
            
        if !seq_scans.is_empty() {
            suggestions.push(format!(
                "Found {} sequential scans - consider adding indexes",
                seq_scans.len()
            ));
        }
        
        // Check for nested loops
        let nested_loops: Vec<_> = plan.operations.iter()
            .filter(|op| op.operation_type == "Nested Loop")
            .collect();
            
        if !nested_loops.is_empty() {
            suggestions.push("Nested loops detected - consider hash joins or indexes".to_string());
        }
        
        // Overall plan cost analysis
        if plan.cost_estimate > self.cost_threshold {
            suggestions.push("High plan cost - consider query rewriting".to_string());
        }
        
        suggestions
    }
    
    fn suggest_optimization(&self, operation: &PlanOperation) -> String {
        match operation.operation_type.as_str() {
            "Seq Scan" => format!(
                "Sequential scan on {} - add index on frequently queried columns",
                operation.table_name
            ),
            "Sort" => "Sort operation detected - consider pre-sorted indexes".to_string(),
            "Hash Join" => "Hash join detected - verify join conditions are indexed".to_string(),
            "Nested Loop" => format!(
                "Nested loop on {} - consider hash join or index optimization",
                operation.table_name
            ),
            _ => format!("Optimize {} operation", operation.operation_type),
        }
    }
    
    pub fn compare_plans(&self, plan1: &QueryPlan, plan2: &QueryPlan) -> (f64, String) {
        let cost_improvement = ((plan1.cost_estimate - plan2.cost_estimate) / plan1.cost_estimate) * 100.0;
        let time_improvement = ((plan1.execution_time - plan2.execution_time) as f64 / plan1.execution_time as f64) * 100.0;
        
        let recommendation = if cost_improvement > 50.0 && time_improvement > 30.0 {
            "Excellent improvement - implement this plan".to_string()
        } else if cost_improvement > 20.0 || time_improvement > 15.0 {
            "Good improvement - consider implementing".to_string()
        } else if cost_improvement > 0.0 || time_improvement > 0.0 {
            "Minor improvement - evaluate carefully".to_string()
        } else {
            "No improvement or regression - keep current plan".to_string()
        };
        
        (cost_improvement, recommendation)
    }
    
    //yaha pe badme plan caching bhi add karna ha
    pub fn identify_plan_patterns(&self, plans: &[QueryPlan]) -> Vec<String> {
        let mut patterns = Vec::new();
        
        // Find common expensive operations
        let mut operation_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for plan in plans {
            for operation in &plan.operations {
                if operation.is_expensive {
                    *operation_counts.entry(operation.operation_type.clone()).or_insert(0) += 1;
                }
            }
        }
        
        for (op_type, count) in operation_counts {
            if count > plans.len() / 2 {
                patterns.push(format!(
                    "{} is frequently expensive across {}% of queries",
                    op_type, (count * 100) / plans.len()
                ));
            }
        }
        
        patterns
    }
}
