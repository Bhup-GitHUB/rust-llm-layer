#[cfg(test)]
mod tests {
    use rust_llm_layer::{PatternAnalyzer, QueryLog, IndexRecommender, PerformancePredictor};

    #[test]
    fn test_pattern_analyzer() {
        let mut analyzer = PatternAnalyzer::new();
        let log = QueryLog::new(
            "SELECT * FROM users".to_string(),
            100,
            1000,
            vec!["users".to_string()],
            1000,
        );
        analyzer.add_log(log);

        let patterns = analyzer.analyze();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].query_type, "SELECT");
    }

    #[test]
    fn test_index_recommender() {
        let recommender = IndexRecommender::new(50.0, 1);
        let patterns = vec![];
        let recommendations = recommender.recommend(&patterns);
        assert_eq!(recommendations.len(), 0);
    }

    #[test]
    fn test_performance_predictor() {
        let predictor = PerformancePredictor::new(true);
        let prediction = predictor.predict("SELECT", 1000);
        assert!(prediction.estimated_time_ms > 0);
    }
}