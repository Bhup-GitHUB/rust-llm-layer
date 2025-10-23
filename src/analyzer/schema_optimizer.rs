use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSchema {
    pub table_name: String,
    pub columns: Vec<ColumnInfo>,
    pub indexes: Vec<String>,
    pub row_count: u64,
    pub avg_row_size: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub is_primary_key: bool,
    pub max_length: Option<u64>,
    pub usage_frequency: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaOptimization {
    pub table_name: String,
    pub optimization_type: String,
    pub description: String,
    pub estimated_benefit: f64,
    pub sql_statement: String,
    pub priority: u32,
}

pub struct SchemaOptimizer {
    data_type_sizes: HashMap<String, u64>,
}

impl SchemaOptimizer {
    pub fn new() -> Self {
        let mut sizes = HashMap::new();
        sizes.insert("INT".to_string(), 4);
        sizes.insert("BIGINT".to_string(), 8);
        sizes.insert("VARCHAR".to_string(), 1);
        sizes.insert("TEXT".to_string(), 4);
        sizes.insert("BOOLEAN".to_string(), 1);
        sizes.insert("DATE".to_string(), 4);
        sizes.insert("TIMESTAMP".to_string(), 8);
        
        Self {
            data_type_sizes: sizes,
        }
    }
    
    //yaha pe badme advanced schema analysis bhi add karna ha
    pub fn analyze_schema(&self, schemas: &[TableSchema]) -> Vec<SchemaOptimization> {
        let mut optimizations = Vec::new();
        
        for schema in schemas {
            // Analyze data types
            optimizations.extend(self.analyze_data_types(schema));
            
            // Analyze table structure
            optimizations.extend(self.analyze_table_structure(schema));
            
            // Analyze normalization opportunities
            optimizations.extend(self.analyze_normalization(schema));
        }
        
        // Sort by priority
        optimizations.sort_by(|a, b| b.priority.cmp(&a.priority));
        optimizations
    }
    
    fn analyze_data_types(&self, schema: &TableSchema) -> Vec<SchemaOptimization> {
        let mut optimizations = Vec::new();
        
        for column in &schema.columns {
            if let Some(optimization) = self.suggest_data_type_optimization(column) {
                optimizations.push(optimization);
            }
        }
        
        optimizations
    }
    
    fn suggest_data_type_optimization(&self, column: &ColumnInfo) -> Option<SchemaOptimization> {
        let current_size = self.estimate_column_size(column);
        let suggested_type = self.get_optimal_data_type(column);
        let suggested_size = self.estimate_type_size(&suggested_type);
        
        if suggested_size < current_size {
            let savings = current_size - suggested_size;
            let benefit = (savings / current_size) * 100.0;
            
            Some(SchemaOptimization {
                table_name: "".to_string(), // Will be set by caller
                optimization_type: "Data Type Optimization".to_string(),
                description: format!(
                    "Change {} from {} to {} - save {} bytes per row",
                    column.name, column.data_type, suggested_type, savings
                ),
                estimated_benefit: benefit,
                sql_statement: format!(
                    "ALTER TABLE {} ALTER COLUMN {} TYPE {}",
                    "", column.name, suggested_type
                ),
                priority: if benefit > 50.0 { 100 } else if benefit > 20.0 { 80 } else { 60 },
            })
        } else {
            None
        }
    }
    
    fn estimate_column_size(&self, column: &ColumnInfo) -> u64 {
        if let Some(max_len) = column.max_length {
            max_len
        } else {
            self.data_type_sizes.get(&column.data_type.to_uppercase()).copied().unwrap_or(8)
        }
    }
    
    fn estimate_type_size(&self, data_type: &str) -> u64 {
        self.data_type_sizes.get(&data_type.to_uppercase()).copied().unwrap_or(8)
    }
    
    fn get_optimal_data_type(&self, column: &ColumnInfo) -> String {
        match column.data_type.to_uppercase().as_str() {
            "VARCHAR" if column.max_length.unwrap_or(0) <= 255 => "TINYTEXT".to_string(),
            "INT" if column.usage_frequency < 1000 => "SMALLINT".to_string(),
            "BIGINT" if column.usage_frequency < 10000 => "INT".to_string(),
            "TEXT" if column.max_length.unwrap_or(0) <= 65535 => "VARCHAR(65535)".to_string(),
            _ => column.data_type.clone(),
        }
    }
    
    fn analyze_table_structure(&self, schema: &TableSchema) -> Vec<SchemaOptimization> {
        let mut optimizations = Vec::new();
        
        // Check for missing primary keys
        let has_primary_key = schema.columns.iter().any(|col| col.is_primary_key);
        if !has_primary_key {
            optimizations.push(SchemaOptimization {
                table_name: schema.table_name.clone(),
                optimization_type: "Primary Key".to_string(),
                description: "Add primary key for better performance".to_string(),
                estimated_benefit: 30.0,
                sql_statement: format!("ALTER TABLE {} ADD COLUMN id SERIAL PRIMARY KEY", schema.table_name),
                priority: 90,
            });
        }
        
        // Check for oversized tables
        if schema.row_count > 1000000 && schema.avg_row_size > 1000.0 {
            optimizations.push(SchemaOptimization {
                table_name: schema.table_name.clone(),
                optimization_type: "Table Partitioning".to_string(),
                description: "Consider partitioning large table".to_string(),
                estimated_benefit: 40.0,
                sql_statement: format!("-- Partition {} by date or range", schema.table_name),
                priority: 85,
            });
        }
        
        optimizations
    }
    
    fn analyze_normalization(&self, schema: &TableSchema) -> Vec<SchemaOptimization> {
        let mut optimizations = Vec::new();
        
        // Check for potential normalization opportunities
        let text_columns: Vec<_> = schema.columns.iter()
            .filter(|col| col.data_type.contains("TEXT") || col.data_type.contains("VARCHAR"))
            .collect();
            
        if text_columns.len() > 5 {
            optimizations.push(SchemaOptimization {
                table_name: schema.table_name.clone(),
                optimization_type: "Normalization".to_string(),
                description: "Consider normalizing table with many text columns".to_string(),
                estimated_benefit: 25.0,
                sql_statement: format!("-- Normalize {} table structure", schema.table_name),
                priority: 70,
            });
        }
        
        optimizations
    }
    
    //yaha pe badme foreign key analysis bhi add karna ha
    pub fn get_schema_health_score(&self, schemas: &[TableSchema]) -> f64 {
        let mut total_score = 0.0;
        let mut table_count = 0;
        
        for schema in schemas {
            let mut table_score = 100.0;
            
            // Deduct for missing primary keys
            let has_primary_key = schema.columns.iter().any(|col| col.is_primary_key);
            if !has_primary_key {
                table_score -= 20.0;
            }
            
            // Deduct for oversized data types
            for column in &schema.columns {
                if self.is_oversized_type(column) {
                    table_score -= 5.0;
                }
            }
            
            // Deduct for too many columns
            if schema.columns.len() > 20 {
                table_score -= 10.0;
            }
            
            total_score += table_score;
            table_count += 1;
        }
        
        if table_count > 0 {
            total_score / table_count as f64
        } else {
            0.0
        }
    }
    
    fn is_oversized_type(&self, column: &ColumnInfo) -> bool {
        match column.data_type.to_uppercase().as_str() {
            "BIGINT" if column.usage_frequency < 10000 => true,
            "TEXT" if column.max_length.unwrap_or(0) < 1000 => true,
            _ => false,
        }
    }
}
