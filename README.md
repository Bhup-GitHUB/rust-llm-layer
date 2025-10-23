# Rust LLM Layer 🦀

A high-performance database query optimization layer built in Rust that uses machine learning techniques to analyze query patterns, predict performance, and recommend database indexes.

## 🚀 Features

- **Query Pattern Analysis**: Automatically identifies slow and frequent query patterns
- **Performance Prediction**: Predicts query execution time based on historical data
- **Index Recommendations**: Suggests optimal database indexes for better performance
- **Real-time Monitoring**: Tracks query metrics and provides insights
- **Efficiency Scoring**: Calculates query efficiency based on rows scanned vs execution time

## 📁 Project Structure

```
rust-llm-layer/
├── src/
│   ├── analyzer/           # Query analysis modules
│   │   ├── query_log.rs    # Query log data structures
│   │   └── pattern_analyzer.rs  # Pattern detection algorithms
│   ├── predictor/          # Performance prediction
│   │   └── performance_predictor.rs
│   ├── recommender/        # Index recommendations
│   │   └── index_recommender.rs
│   ├── lib.rs             # Library exports
│   └── main.rs            # Demo application
├── tests/                 # Integration tests
└── Cargo.toml            # Dependencies
```

## 🛠️ Core Components

### 1. Query Log Analyzer

- Captures query execution metrics
- Identifies query types (SELECT, INSERT, UPDATE, DELETE)
- Calculates efficiency scores
- Tracks table access patterns

### 2. Pattern Analyzer

- Groups queries by type and behavior
- Identifies slow query patterns
- Calculates frequency and performance metrics
- Provides performance summaries

### 3. Performance Predictor

- Uses historical data for predictions
- Considers cache effects
- Provides confidence scores
- Generates optimization recommendations

### 4. Index Recommender

- Analyzes query patterns for optimization opportunities
- Suggests BTree or Hash indexes
- Calculates priority and improvement estimates
- Provides detailed reasoning

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+
- Cargo package manager

### Installation

```bash
# Clone the repository
git clone https://github.com/Bhup-GitHUB/rust-llm-layer.git
cd rust-llm-layer

# Build the project
cargo build

# Run the demo
cargo run
```

### Usage Example

```rust
use rust_llm_layer::{PatternAnalyzer, QueryLog, IndexRecommender, PerformancePredictor};

// Create analyzer
let mut analyzer = PatternAnalyzer::new();

// Add query logs
let log = QueryLog::new(
    "SELECT * FROM users WHERE id = 1".to_string(),
    150,  // execution time in ms
    1000, // timestamp
    vec!["users".to_string()], // tables accessed
    1000, // rows scanned
);

analyzer.add_log(log);

// Analyze patterns
let patterns = analyzer.analyze();

// Get recommendations
let recommender = IndexRecommender::new(100.0, 1);
let recommendations = recommender.recommend(&patterns);

// Predict performance
let mut predictor = PerformancePredictor::new(true);
let prediction = predictor.predict("SELECT", 2000);
```

## 📊 Demo Output

```
=== Rust LLM Layer Demo ===

--- Phase 1: Pattern Analysis ---
Type: SELECT, Frequency: 2, Avg Time: 200.00ms, Slowness: 400.00
Type: INSERT, Frequency: 1, Avg Time: 50.00ms, Slowness: 50.00

--- Phase 2: Index Recommendations ---
Table: users, Column: id, Type: BTree, Priority: 100, Improvement: 40.0%
Reason: Query type: SELECT, Frequency: 2, Avg time: 200.00ms

--- Phase 3: Performance Prediction ---
Estimated Time: 120ms
Confidence: 0.20
Recommendation: Query performance looks good
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture
```

## 📈 Performance Metrics

The system tracks several key metrics:

- **Execution Time**: Query runtime in milliseconds
- **Rows Scanned**: Number of rows processed
- **Efficiency Score**: Performance per row ratio
- **Slowness Score**: Combined frequency and time impact
- **Confidence**: Prediction reliability (0.0 - 1.0)

## 🔧 Configuration

### Index Recommender Settings

- `slowness_threshold`: Minimum slowness score for recommendations
- `frequency_threshold`: Minimum frequency for consideration

### Performance Predictor Settings

- `cache_enabled`: Whether to consider cache effects
- Historical data size affects prediction accuracy

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👨‍💻 Author

**Bhupesh Kumar**

- GitHub: [@Bhup-GitHUB](https://github.com/Bhup-GitHUB)
- Website: [bhupeshkumar.tech](https://www.bhupeshkumar.tech)
- LinkedIn: [Bhupesh Kumar](https://linkedin.com/in/bhupesh-kumar)

## 🙏 Acknowledgments

- Rust community for excellent documentation
- Database optimization research papers
- Open source contributors

## 📚 Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Database Indexing Best Practices](https://use-the-index-luke.com/)
- [Query Optimization Techniques](https://www.postgresql.org/docs/current/query-performance.html)

---

⭐ If you found this project helpful, please give it a star!
