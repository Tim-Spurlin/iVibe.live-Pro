use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Simplified models for the recognition service
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DailyRecognitionSummary {
    pub id: Uuid,
    pub user_name: String,
    pub achievement: String,
    pub city: String,
    pub harmony_score: f64,
    pub scheduled_time: DateTime<Utc>,
    pub recognition_type: String,
}