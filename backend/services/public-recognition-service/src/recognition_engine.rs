use crate::models::*;
use chrono::{DateTime, Utc, Datelike, Weekday};
use sqlx::{PgPool, Row};
use tracing::info;
use uuid::Uuid;

#[derive(Clone)]
pub struct RecognitionEngine {
    db_pool: PgPool,
}

impl RecognitionEngine {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    /// Get all daily recognitions with smart filtering and scheduling
    pub async fn get_daily_recognitions(&self) -> anyhow::Result<Vec<DailyRecognition>> {
        let query = r#"
            SELECT 
                id, user_id, recognition_type, event_description, achievement_details,
                location, scheduled_post_time, actual_post_time, engagement_score,
                harmony_contribution_score, industry_relevance, career_impact_score,
                created_at, status
            FROM daily_recognitions 
            WHERE DATE(scheduled_post_time) = CURRENT_DATE
            ORDER BY harmony_contribution_score DESC, career_impact_score DESC
        "#;

        let rows = sqlx::query(query)
            .fetch_all(&self.db_pool)
            .await?;

        let mut recognitions = Vec::new();
        for row in rows {
            // Note: This is a simplified version - in reality you'd need proper JSON deserialization
            // For now, creating mock data structure
            let recognition = DailyRecognition {
                id: row.get("id"),
                user_id: row.get("user_id"),
                recognition_type: RecognitionType::LocalDaily, // Would parse from DB
                event_description: row.get("event_description"),
                achievement_details: row.get("achievement_details"),
                location: Location {
                    city: "Unknown".to_string(),
                    state_province: "Unknown".to_string(),
                    country: "Unknown".to_string(),
                    latitude: 0.0,
                    longitude: 0.0,
                    timezone: "UTC".to_string(),
                    population: None,
                },
                scheduled_post_time: row.get("scheduled_post_time"),
                actual_post_time: row.get("actual_post_time"),
                engagement_score: row.get("engagement_score"),
                harmony_contribution_score: row.get("harmony_contribution_score"),
                industry_relevance: row.get("industry_relevance"),
                career_impact_score: row.get("career_impact_score"),
                created_at: row.get("created_at"),
                status: RecognitionStatus::Scheduled, // Would parse from DB
            };
            recognitions.push(recognition);
        }

        Ok(recognitions)
    }

    /// Schedule a new recognition with sophisticated algorithms
    pub async fn schedule_recognition(&self, request: ScheduleRecognitionRequest) -> anyhow::Result<DailyRecognition> {
        info!("Scheduling recognition for user: {}", request.user_id);

        // Determine recognition type based on sophisticated timing algorithm
        let recognition_type = self.determine_recognition_type().await;
        
        // Calculate harmony contribution score
        let harmony_score = self.calculate_harmony_contribution(&request).await;
        
        // Calculate career impact score
        let career_impact_score = self.calculate_career_impact(&request).await;
        
        // Determine optimal posting time
        let optimal_time = self.calculate_optimal_posting_time(&request).await?;

        // Check if this recognition should be shown based on local job market needs
        let should_show = self.should_show_recognition(&request).await?;
        
        if !should_show {
            info!("Recognition filtered out due to job market oversaturation in {}", request.location.city);
            // Still create but mark as skipped
        }

        let recognition = DailyRecognition {
            id: Uuid::new_v4(),
            user_id: request.user_id,
            recognition_type,
            event_description: request.event_type,
            achievement_details: request.achievement_description,
            location: request.location,
            scheduled_post_time: optimal_time,
            actual_post_time: None,
            engagement_score: None,
            harmony_contribution_score: harmony_score,
            industry_relevance: request.career_context.as_ref().map(|c| c.industry.clone()),
            career_impact_score,
            created_at: Utc::now(),
            status: if should_show { RecognitionStatus::Scheduled } else { RecognitionStatus::Skipped },
        };

        // Store in database
        self.store_recognition(&recognition).await?;

        info!("Recognition scheduled successfully with harmony score: {:.2}", harmony_score);
        Ok(recognition)
    }

    /// Determine recognition type based on day of week and global scheduling
    async fn determine_recognition_type(&self) -> RecognitionType {
        let now = Utc::now();
        match now.weekday() {
            Weekday::Mon | Weekday::Tue | Weekday::Wed | Weekday::Thu => RecognitionType::LocalDaily,
            Weekday::Fri => RecognitionType::StateWeekly,
            Weekday::Sat => RecognitionType::CountryWeekly,
            Weekday::Sun => RecognitionType::GlobalDiversity,
        }
    }

    /// Calculate harmony contribution using sophisticated algorithms
    async fn calculate_harmony_contribution(&self, request: &ScheduleRecognitionRequest) -> f64 {
        let mut score = 0.0;

        // Base score for positive achievement
        score += 0.3;

        // Career development promotes economic harmony
        if let Some(career_context) = &request.career_context {
            match career_context.career_stage {
                CareerStage::Student => score += 0.2, // Education promotes growth
                CareerStage::EntryLevel => score += 0.25, // New careers reduce unemployment
                CareerStage::MidCareer => score += 0.15,
                CareerStage::Senior => score += 0.1,
                CareerStage::Executive => score += 0.05,
                CareerStage::Retired => score += 0.3, // Wisdom sharing promotes harmony
            }

            // Skills that benefit society score higher
            if career_context.industry.contains("healthcare") ||
               career_context.industry.contains("education") ||
               career_context.industry.contains("public service") {
                score += 0.2;
            }
        }

        // Diversity bonus based on location
        score += self.calculate_diversity_bonus(&request.location).await;

        // Community building events get bonus
        if request.event_type.contains("community") ||
           request.event_type.contains("volunteer") ||
           request.event_type.contains("helping") {
            score += 0.25;
        }

        // Family and life events that promote stability
        if request.event_type.contains("married") ||
           request.event_type.contains("baby") ||
           request.event_type.contains("graduation") {
            score += 0.2;
        }

        // Cap at 1.0 and ensure minimum of 0.1
        score.min(1.0).max(0.1)
    }

    /// Calculate career impact score
    async fn calculate_career_impact(&self, request: &ScheduleRecognitionRequest) -> f64 {
        if let Some(career_context) = &request.career_context {
            // Higher relevance achievements get higher scores
            let base_score = career_context.achievement_relevance;
            
            // Adjust based on skill level - higher skill = more impact
            let skill_multiplier = match career_context.skill_level {
                SkillLevel::Beginner => 0.8,
                SkillLevel::Intermediate => 1.0,
                SkillLevel::Advanced => 1.2,
                SkillLevel::Expert => 1.4,
                SkillLevel::Master => 1.6,
            };

            (base_score * skill_multiplier).min(1.0)
        } else {
            0.5 // Default score for non-career events
        }
    }

    /// Calculate diversity bonus for global harmony
    async fn calculate_diversity_bonus(&self, location: &Location) -> f64 {
        // Check recent recognitions in same location
        let query = r#"
            SELECT COUNT(*) as recent_count
            FROM daily_recognitions 
            WHERE location->>'city' = $1 
            AND created_at > NOW() - INTERVAL '7 days'
        "#;

        if let Ok(row) = sqlx::query(query)
            .bind(&location.city)
            .fetch_one(&self.db_pool)
            .await {
            let count: i64 = row.get("recent_count");
            
            // Less recent activity = higher diversity bonus
            match count {
                0 => 0.3,      // No recent recognition = high bonus
                1 => 0.2,      // One recent = medium bonus
                2..=3 => 0.1,  // Few recent = small bonus
                _ => 0.0,      // Many recent = no bonus
            }
        } else {
            0.15 // Default bonus if query fails
        }
    }

    /// Determine if recognition should be shown based on job market analysis
    async fn should_show_recognition(&self, request: &ScheduleRecognitionRequest) -> anyhow::Result<bool> {
        if let Some(career_context) = &request.career_context {
            // Check if this profession is oversaturated in the area
            let query = r#"
                SELECT demand_level 
                FROM workforce_statistics 
                WHERE city = $1 AND industry = $2
            "#;

            if let Ok(row) = sqlx::query(query)
                .bind(&request.location.city)
                .bind(&career_context.industry)
                .fetch_optional(&self.db_pool)
                .await? {
                
                if let Some(row) = row {
                    let demand_level: String = row.get("demand_level");
                    
                    // Don't show recognition for oversaturated fields
                    return Ok(demand_level != "Saturated");
                }
            }
        }

        // Show by default if no career context or no data
        Ok(true)
    }

    /// Calculate optimal posting time for maximum positive engagement
    async fn calculate_optimal_posting_time(&self, request: &ScheduleRecognitionRequest) -> anyhow::Result<DateTime<Utc>> {
        let now = Utc::now();
        
        // Parse timezone for local time calculation
        let tz: chrono_tz::Tz = request.location.timezone.parse()
            .unwrap_or(chrono_tz::UTC);
        
        let local_now = now.with_timezone(&tz);
        
        // Optimal posting times based on research:
        // - Weekdays: 9-11 AM and 1-3 PM local time
        // - Weekends: 12-2 PM local time
        let optimal_hour = if matches!(local_now.weekday(), Weekday::Sat | Weekday::Sun) {
            13 // 1 PM on weekends
        } else {
            10 // 10 AM on weekdays
        };

        let optimal_local = local_now
            .date_naive()
            .and_hms_opt(optimal_hour, 0, 0)
            .unwrap()
            .and_local_timezone(tz)
            .unwrap();

        Ok(optimal_local.with_timezone(&Utc))
    }

    /// Store recognition in database
    async fn store_recognition(&self, recognition: &DailyRecognition) -> anyhow::Result<()> {
        let query = r#"
            INSERT INTO daily_recognitions (
                id, user_id, recognition_type, event_description, achievement_details,
                location, scheduled_post_time, harmony_contribution_score, 
                industry_relevance, career_impact_score, created_at, status
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        "#;

        sqlx::query(query)
            .bind(&recognition.id)
            .bind(&recognition.user_id)
            .bind(serde_json::to_string(&recognition.recognition_type)?)
            .bind(&recognition.event_description)
            .bind(&recognition.achievement_details)
            .bind(serde_json::to_string(&recognition.location)?)
            .bind(&recognition.scheduled_post_time)
            .bind(&recognition.harmony_contribution_score)
            .bind(&recognition.industry_relevance)
            .bind(&recognition.career_impact_score)
            .bind(&recognition.created_at)
            .bind(serde_json::to_string(&recognition.status)?)
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }

    /// Process daily recognitions - run this as a scheduled job
    pub async fn process_daily_recognitions(&self) -> anyhow::Result<()> {
        info!("Processing daily recognitions...");
        
        // Get all scheduled recognitions for today
        let recognitions = self.get_scheduled_recognitions_for_today().await?;
        
        for recognition in recognitions {
            if recognition.scheduled_post_time <= Utc::now() {
                self.post_recognition(&recognition).await?;
            }
        }
        
        Ok(())
    }

    /// Get scheduled recognitions for today
    async fn get_scheduled_recognitions_for_today(&self) -> anyhow::Result<Vec<DailyRecognition>> {
        // Implementation would fetch from database
        Ok(vec![])
    }

    /// Post recognition to social platforms
    async fn post_recognition(&self, recognition: &DailyRecognition) -> anyhow::Result<()> {
        info!("Posting recognition: {}", recognition.id);
        
        // Here you would implement posting to various social platforms
        // This could include internal platform, external social media, email notifications, etc.
        
        // Update status to posted
        let query = "UPDATE daily_recognitions SET status = 'Posted', actual_post_time = NOW() WHERE id = $1";
        sqlx::query(query)
            .bind(&recognition.id)
            .execute(&self.db_pool)
            .await?;
            
        info!("Recognition posted successfully: {}", recognition.id);
        Ok(())
    }
}