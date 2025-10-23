use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExistingIndex {
    pub table_name: String,
    pub column_names: Vec<String>,
    pub index_name: String,
    pub index_type: String,
    pub is_unique: bool,
    pub is_partial: bool,
    pub filter_condition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexConflict {
    pub recommended_index: String,
    pub existing_index: String,
    pub conflict_type: String, // "duplicate", "overlapping", "redundant"
    pub severity: f64,
}

pub struct ExistingIndexChecker {
    existing_indexes: HashMap<String, Vec<ExistingIndex>>,
}

impl ExistingIndexChecker {
    pub fn new() -> Self {
        Self {
            existing_indexes: HashMap::new(),
        }
    }
    
    //yaha pe badme database connection bhi add karna ha
    pub fn add_existing_index(&mut self, index: ExistingIndex) {
        let table_name = index.table_name.clone();
        self.existing_indexes.entry(table_name).or_insert_with(Vec::new).push(index);
    }
    
    pub fn check_for_conflicts(&self, recommended_table: &str, recommended_columns: &[String]) -> Vec<IndexConflict> {
        let mut conflicts = Vec::new();
        
        if let Some(table_indexes) = self.existing_indexes.get(recommended_table) {
            for existing_index in table_indexes {
                let conflict = self.analyze_index_overlap(existing_index, recommended_columns);
                if let Some(conflict) = conflict {
                    conflicts.push(conflict);
                }
            }
        }
        
        conflicts
    }
    
    fn analyze_index_overlap(&self, existing: &ExistingIndex, recommended: &[String]) -> Option<IndexConflict> {
        let existing_set: std::collections::HashSet<String> = existing.column_names.iter().cloned().collect();
        let recommended_set: std::collections::HashSet<String> = recommended.iter().cloned().collect();
        
        if existing_set == recommended_set {
            return Some(IndexConflict {
                recommended_index: recommended.join(", "),
                existing_index: existing.index_name.clone(),
                conflict_type: "duplicate".to_string(),
                severity: 1.0,
            });
        }
        
        if existing_set.is_superset(&recommended_set) {
            return Some(IndexConflict {
                recommended_index: recommended.join(", "),
                existing_index: existing.index_name.clone(),
                conflict_type: "redundant".to_string(),
                severity: 0.8,
            });
        }
        
        if !existing_set.is_disjoint(&recommended_set) {
            return Some(IndexConflict {
                recommended_index: recommended.join(", "),
                existing_index: existing.index_name.clone(),
                conflict_type: "overlapping".to_string(),
                severity: 0.6,
            });
        }
        
        None
    }
    
    pub fn get_consolidation_suggestions(&self) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        for (table_name, indexes) in &self.existing_indexes {
            if indexes.len() > 3 {
                suggestions.push(format!(
                    "Table '{}' has {} indexes - consider consolidation",
                    table_name, indexes.len()
                ));
            }
            
            let duplicate_columns: Vec<_> = indexes.iter()
                .filter(|idx| idx.column_names.len() == 1)
                .map(|idx| &idx.column_names[0])
                .collect();
                
            if duplicate_columns.len() > 1 {
                suggestions.push(format!(
                    "Table '{}' has multiple single-column indexes on similar columns",
                    table_name
                ));
            }
        }
        
        suggestions
    }
    
    //yaha pe badme index statistics bhi add karna ha
    pub fn get_index_statistics(&self) -> (usize, usize, usize) {
        let total_indexes: usize = self.existing_indexes.values().map(|v| v.len()).sum();
        let unique_indexes = self.existing_indexes.values()
            .flatten()
            .filter(|idx| idx.is_unique)
            .count();
        let partial_indexes = self.existing_indexes.values()
            .flatten()
            .filter(|idx| idx.is_partial)
            .count();
            
        (total_indexes, unique_indexes, partial_indexes)
    }
}
