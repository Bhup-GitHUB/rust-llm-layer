use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedQuery {
    pub where_clauses: Vec<String>,
    pub join_conditions: Vec<String>,
    pub order_by_columns: Vec<String>,
    pub select_columns: Vec<String>,
    pub from_tables: Vec<String>,
    pub query_fingerprint: String,
}

impl ParsedQuery {
    pub fn new() -> Self {
        Self {
            where_clauses: Vec::new(),
            join_conditions: Vec::new(),
            order_by_columns: Vec::new(),
            select_columns: Vec::new(),
            from_tables: Vec::new(),
            query_fingerprint: String::new(),
        }
    }
}

pub struct QueryParser;

impl QueryParser {
    //yaha pe badme complex SQL parsing bhi add karna ha
    pub fn parse(&self, query: &str) -> ParsedQuery {
        let query_upper = query.to_uppercase();
        let mut parsed = ParsedQuery::new();
        
        if let Some(where_start) = query_upper.find("WHERE") {
            if let Some(where_end) = self.find_clause_end(&query[where_start..]) {
                let where_clause = &query[where_start + 5..where_start + where_end];
                parsed.where_clauses = self.extract_conditions(where_clause);
            }
        }
        
        parsed.join_conditions = self.extract_joins(query);
        
        if let Some(order_start) = query_upper.find("ORDER BY") {
            if let Some(order_end) = self.find_clause_end(&query[order_start..]) {
                let order_clause = &query[order_start + 9..order_start + order_end];
                parsed.order_by_columns = self.extract_columns(order_clause);
            }
        }
        
        if let Some(select_start) = query_upper.find("SELECT") {
            if let Some(from_start) = query_upper.find("FROM") {
                let select_clause = &query[select_start + 6..from_start];
                parsed.select_columns = self.extract_columns(select_clause);
            }
        }
        
        if let Some(from_start) = query_upper.find("FROM") {
            if let Some(where_start) = query_upper.find("WHERE") {
                let from_clause = &query[from_start + 4..where_start];
                parsed.from_tables = self.extract_tables(from_clause);
            } else if let Some(order_start) = query_upper.find("ORDER BY") {
                let from_clause = &query[from_start + 4..order_start];
                parsed.from_tables = self.extract_tables(from_clause);
            } else {
                parsed.from_tables = self.extract_tables(&query[from_start + 4..]);
            }
        }
        
        parsed.query_fingerprint = self.generate_fingerprint(&parsed);
        
        parsed
    }
    
    fn extract_conditions(&self, where_clause: &str) -> Vec<String> {
        where_clause
            .split("AND")
            .chain(where_clause.split("OR"))
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
    
    fn extract_joins(&self, query: &str) -> Vec<String> {
        let query_upper = query.to_uppercase();
        let mut joins = Vec::new();
        
        let mut start = 0;
        while let Some(join_pos) = query_upper[start..].find("JOIN") {
            let actual_pos = start + join_pos;
            if let Some(end) = self.find_clause_end(&query[actual_pos..]) {
                let join_clause = &query[actual_pos..actual_pos + end];
                joins.push(join_clause.trim().to_string());
                start = actual_pos + end;
            } else {
                break;
            }
        }
        
        joins
    }
    
    fn extract_columns(&self, clause: &str) -> Vec<String> {
        clause
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
    
    fn extract_tables(&self, clause: &str) -> Vec<String> {
        clause
            .split(',')
            .map(|s| s.trim().split_whitespace().next().unwrap_or("").to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
    
    fn find_clause_end(&self, text: &str) -> Option<usize> {
        let keywords = ["WHERE", "ORDER BY", "GROUP BY", "HAVING", "LIMIT"];
        let mut min_pos = None;
        
        for keyword in keywords {
            if let Some(pos) = text[1..].find(keyword) {
                let actual_pos = pos + 1;
                min_pos = Some(min_pos.map_or(actual_pos, |m| m.min(actual_pos)));
            }
        }
        
        min_pos
    }
    
    fn generate_fingerprint(&self, parsed: &ParsedQuery) -> String {
        let mut fingerprint = String::new();
        fingerprint.push_str(&format!("SELECT_{}", parsed.select_columns.len()));
        fingerprint.push_str(&format!("_FROM_{}", parsed.from_tables.len()));
        fingerprint.push_str(&format!("_WHERE_{}", parsed.where_clauses.len()));
        fingerprint.push_str(&format!("_JOIN_{}", parsed.join_conditions.len()));
        fingerprint.push_str(&format!("_ORDER_{}", parsed.order_by_columns.len()));
        fingerprint
    }
}
