use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Time-based query patterns - ye struct time-based patterns store karta hai
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimePattern {
    pub hour: u8,
    pub day_of_week: u8, // 0 = Sunday, 1 = Monday, etc.
    pub query_count: u64,
    pub avg_execution_time: f64,
    pub peak_performance: bool,
}

/// Time-based analysis for query patterns - ye class time-based analysis karta hai
pub struct TimeAnalyzer {
    hourly_stats: HashMap<u8, (u64, f64)>, // hour -> (count, total_time)
    daily_stats: HashMap<u8, (u64, f64)>,  // day -> (count, total_time)
}

impl TimeAnalyzer {
    pub fn new() -> Self {
        Self {
            hourly_stats: HashMap::new(),
            daily_stats: HashMap::new(),
        }
    }
    
    /// Analyze query by timestamp - ye method timestamp ke basis pe analyze karta hai
    pub fn analyze_timestamp(&mut self, timestamp: u64, execution_time: u64) {
        let datetime = self.timestamp_to_datetime(timestamp);
        
        // Update hourly stats - ye important hai peak time identify karne ke liye
        let hour_entry = self.hourly_stats.entry(datetime.hour).or_insert((0, 0.0));
        hour_entry.0 += 1;
        hour_entry.1 += execution_time as f64;
        
        // Update daily stats
        let day_entry = self.daily_stats.entry(datetime.day_of_week).or_insert((0, 0.0));
        day_entry.0 += 1;
        day_entry.1 += execution_time as f64;
    }
    
    fn timestamp_to_datetime(&self, timestamp: u64) -> DateTimeInfo {
        // Simple timestamp conversion - ye basic conversion hai
        let seconds = timestamp / 1000;
        let days_since_epoch = seconds / 86400;
        let day_of_week = (days_since_epoch + 4) % 7; // Jan 1, 1970 was Thursday (4)
        let hour = (seconds % 86400) / 3600;
        
        DateTimeInfo {
            hour: hour as u8,
            day_of_week: day_of_week as u8,
        }
    }
    
    /// Get peak hours - ye method peak hours identify karta hai
    pub fn get_peak_hours(&self) -> Vec<TimePattern> {
        let mut patterns = Vec::new();
        
        for (hour, (count, total_time)) in &self.hourly_stats {
            let avg_time = total_time / *count as f64;
            let is_peak = *count > 10 && avg_time > 100.0; // Peak criteria
            
            patterns.push(TimePattern {
                hour: *hour,
                day_of_week: 0, // Will be filled separately
                query_count: *count,
                avg_execution_time: avg_time,
                peak_performance: is_peak,
            });
        }
        
        patterns.sort_by(|a, b| b.query_count.cmp(&a.query_count));
        patterns
    }
    
    /// Get daily patterns - ye method daily patterns analyze karta hai
    pub fn get_daily_patterns(&self) -> Vec<TimePattern> {
        let mut patterns = Vec::new();
        
        for (day, (count, total_time)) in &self.daily_stats {
            let avg_time = total_time / *count as f64;
            let is_peak = *count > 50; // Daily peak criteria
            
            patterns.push(TimePattern {
                hour: 0, // Will be filled separately
                day_of_week: *day,
                query_count: *count,
                avg_execution_time: avg_time,
                peak_performance: is_peak,
            });
        }
        
        patterns.sort_by(|a, b| b.query_count.cmp(&a.query_count));
        patterns
    }
    
    /// Get performance recommendations based on time - ye method time-based recommendations deta hai
    pub fn get_time_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // Check for peak hours
        let peak_hours = self.get_peak_hours();
        if !peak_hours.is_empty() {
            let peak_hour = peak_hours[0].hour;
            recommendations.push(format!(
                "Peak query activity detected at hour {} - consider load balancing",
                peak_hour
            ));
        }
        
        // Check for slow periods
        let slow_hours: Vec<_> = self.hourly_stats
            .iter()
            .filter(|(_, (_, total_time))| *total_time > 1000.0)
            .map(|(hour, _)| *hour)
            .collect();
            
        if !slow_hours.is_empty() {
            recommendations.push(format!(
                "Slow query periods detected at hours: {:?} - review indexing strategy",
                slow_hours
            ));
        }
        
        recommendations
    }
}

struct DateTimeInfo {
    hour: u8,
    day_of_week: u8,
}
