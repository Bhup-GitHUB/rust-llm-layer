use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Column usage statistics - ye struct column usage track karta hai
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnUsage {
    pub column_name: String,
    pub table_name: String,
    pub usage_count: u64,
    pub in_where_clause: u64,
    pub in_join_condition: u64,
    pub in_order_by: u64,
    pub avg_query_time: f64,
}

impl ColumnUsage {
    pub fn new(column_name: String, table_name: String) -> Self {
        Self {
            column_name,
            table_name,
            usage_count: 0,
            in_where_clause: 0,
            in_join_condition: 0,
            in_order_by: 0,
            avg_query_time: 0.0,
        }
    }
}

/// Tracks column usage patterns - ye class column usage track karta hai
pub struct ColumnTracker {
    column_stats: HashMap<String, ColumnUsage>,
}

impl ColumnTracker {
    pub fn new() -> Self {
        Self {
            column_stats: HashMap::new(),
        }
    }
    
    /// Track column usage from parsed query - ye method column usage track karta hai
    pub fn track_usage(&mut self, parsed_query: &crate::analyzer::query_parser::ParsedQuery, execution_time: u64) {
        // Track WHERE clause columns - ye important hai performance ke liye
        for where_clause in &parsed_query.where_clauses {
            if let Some(column) = self.extract_column_from_condition(where_clause) {
                self.update_column_stats(&column, "WHERE", execution_time);
            }
        }
        
        // Track JOIN condition columns
        for join_clause in &parsed_query.join_conditions {
            if let Some(column) = self.extract_column_from_condition(join_clause) {
                self.update_column_stats(&column, "JOIN", execution_time);
            }
        }
        
        // Track ORDER BY columns
        for column in &parsed_query.order_by_columns {
            self.update_column_stats(column, "ORDER_BY", execution_time);
        }
    }
    
    fn extract_column_from_condition(&self, condition: &str) -> Option<String> {
        // Simple column extraction - ye basic pattern matching hai
        if let Some(equals_pos) = condition.find('=') {
            let left_side = &condition[..equals_pos].trim();
            if left_side.contains('.') {
                return Some(left_side.to_string());
            }
        }
        None
    }
    
    fn update_column_stats(&mut self, column: &str, usage_type: &str, execution_time: u64) {
        let key = column.to_string();
        let entry = self.column_stats.entry(key.clone()).or_insert_with(|| {
            let (table, col) = if column.contains('.') {
                let parts: Vec<&str> = column.split('.').collect();
                (parts[0].to_string(), parts[1].to_string())
            } else {
                ("unknown".to_string(), column.to_string())
            };
            ColumnUsage::new(col, table)
        });
        
        entry.usage_count += 1;
        match usage_type {
            "WHERE" => entry.in_where_clause += 1,
            "JOIN" => entry.in_join_condition += 1,
            "ORDER_BY" => entry.in_order_by += 1,
            _ => {}
        }
        
        // Update average query time - ye performance tracking ke liye important hai
        let total_time = entry.avg_query_time * (entry.usage_count - 1) as f64 + execution_time as f64;
        entry.avg_query_time = total_time / entry.usage_count as f64;
    }
    
    /// Get most frequently used columns - ye method sabse zyada use hone wale columns deta hai
    pub fn get_most_used_columns(&self, limit: usize) -> Vec<&ColumnUsage> {
        let mut columns: Vec<&ColumnUsage> = self.column_stats.values().collect();
        columns.sort_by(|a, b| b.usage_count.cmp(&a.usage_count));
        columns.into_iter().take(limit).collect()
    }
    
    /// Get columns that need indexing - ye method indexing ke liye best columns suggest karta hai
    pub fn get_indexing_candidates(&self) -> Vec<&ColumnUsage> {
        self.column_stats
            .values()
            .filter(|col| col.in_where_clause > 0 || col.in_join_condition > 0)
            .filter(|col| col.usage_count >= 5) // Minimum usage threshold
            .collect()
    }
    
    /// Get performance summary - ye method performance summary deta hai
    pub fn get_performance_summary(&self) -> (f64, usize, usize) {
        let total_columns = self.column_stats.len();
        let indexed_candidates = self.get_indexing_candidates().len();
        let avg_usage = if total_columns > 0 {
            self.column_stats.values().map(|c| c.usage_count as f64).sum::<f64>() / total_columns as f64
        } else {
            0.0
        };
        
        (avg_usage, total_columns, indexed_candidates)
    }
}
