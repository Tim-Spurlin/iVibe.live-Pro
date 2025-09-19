use crate::models::*;
use chrono::{DateTime, Utc, Datelike, Weekday, Timelike, TimeZone};
use sqlx::{PgPool, Row};
use tracing::info;
use nalgebra::{DVector, DMatrix};

/// Advanced Scheduling Optimizer using machine learning and mathematical optimization
/// 
/// This service implements sophisticated algorithms to determine the optimal posting
/// times for maximum positive engagement, harmony impact, and global coordination.
#[derive(Clone)]
pub struct SchedulingOptimizer {
    db_pool: PgPool,
    engagement_model: EngagementModel,
    harmony_weights: DMatrix<f64>,
}

#[derive(Clone)]
pub struct EngagementModel {
    /// Learned weights for different time factors
    pub time_weights: DVector<f64>,
    /// Learned weights for content factors  
    pub content_weights: DVector<f64>,
    /// Learned weights for audience factors
    pub audience_weights: DVector<f64>,
    /// Global engagement baseline
    pub baseline_engagement: f64,
}

impl SchedulingOptimizer {
    pub fn new(db_pool: PgPool) -> Self {
        let engagement_model = Self::initialize_engagement_model();
        let harmony_weights = Self::initialize_harmony_weights();
        
        Self {
            db_pool,
            engagement_model,
            harmony_weights,
        }
    }

    /// Calculate optimal posting time using advanced ML algorithms
    pub async fn calculate_optimal_posting_time(
        &self,
        timezone: &str,
        event_type: &str,
        target_audience: &TargetAudience,
    ) -> DateTime<Utc> {
        info!("Calculating optimal posting time for {} in {}", event_type, timezone);

        // Get historical engagement data for this pattern
        let historical_data = self.get_historical_engagement_data(
            timezone, 
            event_type, 
            target_audience
        ).await.unwrap_or_default();

        // Calculate engagement probabilities for all possible time slots
        let time_slots = self.generate_candidate_time_slots(timezone).await;
        let mut scored_slots = Vec::new();

        for time_slot in time_slots {
            let engagement_score = self.predict_engagement_score(
                &time_slot,
                event_type,
                target_audience,
                &historical_data,
            ).await;

            let harmony_score = self.calculate_harmony_timing_score(
                &time_slot,
                event_type,
                timezone,
            ).await;

            let global_coordination_score = self.calculate_global_coordination_score(
                &time_slot,
                timezone,
            ).await;

            // Combined optimization score using weighted factors
            let total_score = (engagement_score * 0.4) + 
                             (harmony_score * 0.4) + 
                             (global_coordination_score * 0.2);

            scored_slots.push((time_slot, total_score));
        }

        // Sort by score and return the best time
        scored_slots.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        scored_slots.first()
            .map(|(time, _)| *time)
            .unwrap_or_else(|| Utc::now())
    }

    /// Generate candidate time slots for optimization
    async fn generate_candidate_time_slots(&self, timezone: &str) -> Vec<DateTime<Utc>> {
        let tz: chrono_tz::Tz = timezone.parse().unwrap_or(chrono_tz::UTC);
        let now_local = Utc::now().with_timezone(&tz);
        let mut slots = Vec::new();

        // Generate slots for next 7 days at 1-hour intervals
        for day_offset in 0..7 {
            let target_date = now_local.date_naive() + chrono::Duration::days(day_offset);
            
            for hour in 6..24 { // 6 AM to 11 PM local time
                if let Some(naive_time) = chrono::NaiveTime::from_hms_opt(hour, 0, 0) {
                    let naive_datetime = target_date.and_time(naive_time);
                    if let Some(local_datetime) = tz.from_local_datetime(&naive_datetime).single() {
                        slots.push(local_datetime.with_timezone(&Utc));
                    }
                }
            }
        }

        slots
    }

    /// Predict engagement score using machine learning model
    async fn predict_engagement_score(
        &self,
        time_slot: &DateTime<Utc>,
        event_type: &str,
        target_audience: &TargetAudience,
        historical_data: &[HistoricalEngagement],
    ) -> f64 {
        // Extract time features
        let time_features = self.extract_time_features(time_slot);
        
        // Extract content features
        let content_features = self.extract_content_features(event_type);
        
        // Extract audience features
        let audience_features = self.extract_audience_features(target_audience);

        // Predict using linear model (in practice, would use more sophisticated ML)
        let time_score = time_features.dot(&self.engagement_model.time_weights);
        let content_score = content_features.dot(&self.engagement_model.content_weights);
        let audience_score = audience_features.dot(&self.engagement_model.audience_weights);

        // Historical pattern matching
        let historical_score = self.calculate_historical_pattern_score(time_slot, historical_data);

        // Combine scores with baseline
        let prediction = self.engagement_model.baseline_engagement + 
                        time_score + content_score + audience_score + historical_score;

        // Apply sigmoid to bound between 0 and 1
        1.0 / (1.0 + (-prediction).exp())
    }

    /// Extract time-based features for ML model
    fn extract_time_features(&self, time_slot: &DateTime<Utc>) -> DVector<f64> {
        let hour = time_slot.hour() as f64;
        let day_of_week = time_slot.weekday().number_from_monday() as f64;
        let day_of_month = time_slot.day() as f64;
        let month = time_slot.month() as f64;
        
        // Cyclical encoding for time features
        let hour_sin = (2.0 * std::f64::consts::PI * hour / 24.0).sin();
        let hour_cos = (2.0 * std::f64::consts::PI * hour / 24.0).cos();
        let dow_sin = (2.0 * std::f64::consts::PI * day_of_week / 7.0).sin();
        let dow_cos = (2.0 * std::f64::consts::PI * day_of_week / 7.0).cos();
        let month_sin = (2.0 * std::f64::consts::PI * month / 12.0).sin();
        let month_cos = (2.0 * std::f64::consts::PI * month / 12.0).cos();

        // Weekend indicator
        let is_weekend = if matches!(time_slot.weekday(), Weekday::Sat | Weekday::Sun) { 1.0 } else { 0.0 };

        DVector::from_vec(vec![
            hour_sin, hour_cos, dow_sin, dow_cos, month_sin, month_cos,
            is_weekend, day_of_month / 31.0, // Normalized day of month
        ])
    }

    /// Extract content-based features
    fn extract_content_features(&self, event_type: &str) -> DVector<f64> {
        let mut features = vec![0.0; 10]; // 10 content feature dimensions

        // Binary features for event types
        if event_type.contains("achievement") { features[0] = 1.0; }
        if event_type.contains("celebration") { features[1] = 1.0; }
        if event_type.contains("career") { features[2] = 1.0; }
        if event_type.contains("family") { features[3] = 1.0; }
        if event_type.contains("community") { features[4] = 1.0; }
        if event_type.contains("volunteer") { features[5] = 1.0; }
        if event_type.contains("education") { features[6] = 1.0; }
        if event_type.contains("health") { features[7] = 1.0; }
        if event_type.contains("creative") { features[8] = 1.0; }
        
        // Positivity score (simple keyword-based)
        let positivity_score = self.calculate_positivity_score(event_type);
        features[9] = positivity_score;

        DVector::from_vec(features)
    }

    /// Extract audience-based features
    fn extract_audience_features(&self, target_audience: &TargetAudience) -> DVector<f64> {
        let mut features = vec![0.0; 8]; // 8 audience feature dimensions

        // Age group encoding
        for age_group in &target_audience.age_groups {
            match age_group {
                AgeGroup::Gen_Z => features[0] = 1.0,
                AgeGroup::Millennial => features[1] = 1.0,
                AgeGroup::Gen_X => features[2] = 1.0,
                AgeGroup::Boomer => features[3] = 1.0,
            }
        }

        // Interest diversity
        features[4] = (target_audience.interests.len() as f64 / 10.0).min(1.0);

        // Career field diversity
        features[5] = (target_audience.career_fields.len() as f64 / 10.0).min(1.0);

        // Geographic scope encoding
        features[6] = match target_audience.geographic_scope {
            GeographicScope::Neighborhood(_) => 0.2,
            GeographicScope::City => 0.4,
            GeographicScope::Metropolitan => 0.6,
            GeographicScope::State => 0.8,
            GeographicScope::Country => 1.0,
            GeographicScope::Global => 1.2,
        };

        // Audience size estimate (simplified)
        features[7] = 0.5; // Default moderate audience size

        DVector::from_vec(features)
    }

    /// Calculate positivity score for content
    fn calculate_positivity_score(&self, event_type: &str) -> f64 {
        let positive_keywords = [
            "achievement", "success", "celebration", "joy", "happiness",
            "love", "kindness", "help", "support", "community", "unity",
            "peace", "harmony", "growth", "improvement", "victory",
            "accomplishment", "milestone", "breakthrough", "progress"
        ];

        let negative_keywords = [
            "problem", "issue", "difficulty", "challenge", "struggle",
            "loss", "failure", "conflict", "dispute", "crisis"
        ];

        let mut score: f64 = 0.5; // Neutral baseline

        for keyword in &positive_keywords {
            if event_type.to_lowercase().contains(keyword) {
                score += 0.05;
            }
        }

        for keyword in &negative_keywords {
            if event_type.to_lowercase().contains(keyword) {
                score -= 0.05;
            }
        }

        score.max(0.0).min(1.0)
    }

    /// Calculate harmony timing score
    async fn calculate_harmony_timing_score(
        &self,
        time_slot: &DateTime<Utc>,
        event_type: &str,
        _timezone: &str,
    ) -> f64 {
        // Base harmony score varies by time of day
        let hour = time_slot.hour();
        let base_harmony = match hour {
            6..=8 => 0.8,   // Morning: fresh start, positive energy
            9..=11 => 0.9,  // Mid-morning: peak alertness and positivity
            12..=13 => 0.7, // Lunch: moderate harmony
            14..=16 => 0.85, // Afternoon: sustained positive energy
            17..=19 => 0.95, // Evening: peak social time
            20..=22 => 0.8,  // Night: family and reflection time
            _ => 0.5,       // Late night/early morning: lower harmony
        };

        // Event type harmony modifiers
        let event_harmony_bonus = match event_type {
            t if t.contains("peace") || t.contains("unity") => 0.2,
            t if t.contains("community") || t.contains("helping") => 0.15,
            t if t.contains("celebration") || t.contains("joy") => 0.1,
            t if t.contains("achievement") || t.contains("success") => 0.1,
            t if t.contains("family") || t.contains("love") => 0.15,
            _ => 0.0,
        };

        // Timezone-specific harmony adjustments
        let timezone_harmony = self.get_timezone_harmony_factor(timezone, time_slot).await;

        // Global harmony synchronization bonus
        let sync_bonus = self.calculate_global_harmony_sync_bonus(time_slot).await;

        (base_harmony + event_harmony_bonus + timezone_harmony + sync_bonus).min(1.0)
    }

    /// Calculate global coordination score for synchronized posting
    async fn calculate_global_coordination_score(
        &self,
        time_slot: &DateTime<Utc>,
        timezone: &str,
    ) -> f64 {
        // Check if this time aligns with global recognition waves
        let hour_utc = time_slot.hour();
        
        // Optimal UTC hours for global coordination (when most timezones are active)
        let coordination_score = match hour_utc {
            12..=16 => 0.9, // Peak global activity window
            8..=11 => 0.8,  // Morning wave
            17..=20 => 0.85, // Evening wave
            _ => 0.5,       // Off-peak coordination
        };

        // Bonus for participating in global waves
        let wave_participation_bonus = self.calculate_wave_participation_bonus(time_slot).await;

        (coordination_score + wave_participation_bonus).min(1.0)
    }

    /// Get historical engagement data for pattern matching
    async fn get_historical_engagement_data(
        &self,
        timezone: &str,
        event_type: &str,
        _target_audience: &TargetAudience,
    ) -> anyhow::Result<Vec<HistoricalEngagement>> {
        let query = r#"
            SELECT 
                posted_time,
                engagement_score,
                positive_reactions,
                total_reactions,
                reach,
                event_type
            FROM historical_engagement_data 
            WHERE timezone = $1 
            AND event_type ILIKE $2
            AND posted_time > NOW() - INTERVAL '90 days'
            ORDER BY posted_time DESC
            LIMIT 100
        "#;

        let rows = sqlx::query(query)
            .bind(timezone)
            .bind(format!("%{}%", event_type))
            .fetch_all(&self.db_pool)
            .await?;

        let mut data = Vec::new();
        for row in rows {
            data.push(HistoricalEngagement {
                posted_time: row.get("posted_time"),
                engagement_score: row.get("engagement_score"),
                positive_reactions: row.get("positive_reactions"),
                total_reactions: row.get("total_reactions"),
                reach: row.get("reach"),
                event_type: row.get("event_type"),
            });
        }

        Ok(data)
    }

    /// Calculate historical pattern matching score
    fn calculate_historical_pattern_score(
        &self,
        time_slot: &DateTime<Utc>,
        historical_data: &[HistoricalEngagement],
    ) -> f64 {
        if historical_data.is_empty() {
            return 0.0;
        }

        let target_hour = time_slot.hour();
        let target_weekday = time_slot.weekday();

        let mut matching_scores = Vec::new();

        for data_point in historical_data {
            let data_hour = data_point.posted_time.hour();
            let data_weekday = data_point.posted_time.weekday();

            // Calculate similarity score
            let hour_similarity = 1.0 - (target_hour as i32 - data_hour as i32).abs() as f64 / 24.0;
            let weekday_similarity = if target_weekday == data_weekday { 1.0 } else { 0.7 };

            let time_similarity = (hour_similarity + weekday_similarity) / 2.0;
            let weighted_engagement = data_point.engagement_score * time_similarity;

            matching_scores.push(weighted_engagement);
        }

        // Return weighted average
        if !matching_scores.is_empty() {
            matching_scores.iter().sum::<f64>() / matching_scores.len() as f64
        } else {
            0.0
        }
    }

    /// Get timezone-specific harmony factor
    async fn get_timezone_harmony_factor(&self, timezone: &str, _time_slot: &DateTime<Utc>) -> f64 {
        // Cultural harmony adjustments by region
        match timezone {
            tz if tz.contains("Asia/") => 0.1,     // Higher harmony values for Asian cultures
            tz if tz.contains("Europe/") => 0.05,  // Moderate harmony bonus for European cultures
            tz if tz.contains("America/") => 0.0,  // Neutral for American timezones
            tz if tz.contains("Africa/") => 0.08,  // Community-oriented cultures bonus
            _ => 0.0,
        }
    }

    /// Calculate global harmony synchronization bonus
    async fn calculate_global_harmony_sync_bonus(&self, time_slot: &DateTime<Utc>) -> f64 {
        // Check if this time slot participates in global harmony synchronization
        // This would involve checking against global recognition schedules

        let hour_utc = time_slot.hour();
        
        // Hours that align with global recognition waves get bonus
        match hour_utc {
            0 | 6 | 12 | 18 => 0.1, // Every 6 hours: global sync points
            3 | 9 | 15 | 21 => 0.05, // Mid-points: secondary sync
            _ => 0.0,
        }
    }

    /// Calculate wave participation bonus
    async fn calculate_wave_participation_bonus(&self, _time_slot: &DateTime<Utc>) -> f64 {
        // This would calculate bonus for participating in coordinated global recognition waves
        // For now, return a small bonus to encourage coordination
        0.05
    }

    /// Initialize machine learning engagement model
    fn initialize_engagement_model() -> EngagementModel {
        EngagementModel {
            // Initialize with learned weights (in practice, these would be trained from data)
            time_weights: DVector::from_vec(vec![0.1, 0.1, 0.2, 0.2, 0.05, 0.05, 0.3, 0.1]),
            content_weights: DVector::from_vec(vec![0.1, 0.15, 0.08, 0.12, 0.2, 0.18, 0.07, 0.05, 0.03, 0.02]),
            audience_weights: DVector::from_vec(vec![0.1, 0.12, 0.08, 0.06, 0.15, 0.12, 0.25, 0.12]),
            baseline_engagement: 0.3, // 30% baseline engagement
        }
    }

    /// Initialize harmony weighting matrix
    fn initialize_harmony_weights() -> DMatrix<f64> {
        // 5x5 matrix for different harmony factors
        DMatrix::from_row_slice(5, 5, &[
            1.0, 0.8, 0.6, 0.4, 0.2, // Unity weights
            0.8, 1.0, 0.7, 0.5, 0.3, // Peace weights  
            0.6, 0.7, 1.0, 0.8, 0.4, // Diversity weights
            0.4, 0.5, 0.8, 1.0, 0.6, // Community weights
            0.2, 0.3, 0.4, 0.6, 1.0, // Trust weights
        ])
    }

    /// Update model based on engagement feedback (for continuous learning)
    pub async fn update_model_with_feedback(
        &mut self,
        posted_time: &DateTime<Utc>,
        actual_engagement: f64,
        event_type: &str,
        target_audience: &TargetAudience,
    ) -> anyhow::Result<()> {
        // Store the feedback for model retraining
        let query = r#"
            INSERT INTO engagement_feedback (
                posted_time, actual_engagement, event_type, 
                target_audience, created_at
            ) VALUES ($1, $2, $3, $4, NOW())
        "#;

        sqlx::query(query)
            .bind(posted_time)
            .bind(actual_engagement)
            .bind(event_type)
            .bind(serde_json::to_string(target_audience)?)
            .execute(&self.db_pool)
            .await?;

        // In a production system, this would trigger model retraining
        info!("Engagement feedback stored for model improvement");

        Ok(())
    }
}

#[derive(Debug)]
pub struct HistoricalEngagement {
    pub posted_time: DateTime<Utc>,
    pub engagement_score: f64,
    pub positive_reactions: i32,
    pub total_reactions: i32,
    pub reach: i32,
    pub event_type: String,
}