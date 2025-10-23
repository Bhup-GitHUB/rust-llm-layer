use rust_llm_layer::{
    PatternAnalyzer, QueryLog, IndexRecommender, PerformancePredictor
};

fn main() {
    println!("=== Rust LLM Layer Demo ===\n");

    let mut analyzer = PatternAnalyzer::new();

    //yaha pe badme real database connection bhi add karna ha
    let log1 = QueryLog::new(
        "SELECT * FROM users WHERE id = 1".to_string(),
        150,
        1000,
        vec!["users".to_string()],
        1000,
    );

    let log2 = QueryLog::new(
        "SELECT * FROM users WHERE email = 'test@example.com'".to_string(),
        250,
        2000,
        vec!["users".to_string()],
        5000,
    );

    let log3 = QueryLog::new(
        "INSERT INTO orders VALUES (1, 'product')".to_string(),
        50,
        3000,
        vec!["orders".to_string()],
        1,
    );

    analyzer.add_log(log1.clone());
    analyzer.add_log(log2.clone());
    analyzer.add_log(log3.clone());

    println!("--- Phase 1: Pattern Analysis ---");
    let patterns = analyzer.analyze();
    for pattern in &patterns {
        println!(
            "Type: {}, Frequency: {}, Avg Time: {:.2}ms, Slowness: {:.2}",
            pattern.query_type,
            pattern.frequency,
            pattern.avg_execution_time_ms,
            pattern.slowness_score
        );
    }

    println!("\n--- Phase 2: Index Recommendations ---");
    let recommender = IndexRecommender::new(100.0, 1);
    let recommendations = recommender.recommend(&patterns);
    for rec in &recommendations {
        println!(
            "Table: {}, Column: {}, Type: {:?}, Priority: {}, Improvement: {:.1}%",
            rec.table, rec.column, rec.index_type, rec.priority, rec.estimated_improvement_percent
        );
        println!("Reason: {}\n", rec.reason);
    }

    println!("--- Phase 3: Performance Prediction ---");
    let mut predictor = PerformancePredictor::new(true);
    predictor.add_historical_data(log1);
    predictor.add_historical_data(log2);

    let prediction = predictor.predict("SELECT", 2000);
    println!("Estimated Time: {}ms", prediction.estimated_time_ms);
    println!("Confidence: {:.2}", prediction.confidence);
    println!("Recommendation: {}", prediction.recommendation);
    
    //yaha pe badme export functionality bhi add karna ha
    //function export_to_json() {}
    //function export_to_csv() {}
}
