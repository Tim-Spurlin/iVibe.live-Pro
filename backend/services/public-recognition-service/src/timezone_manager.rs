use chrono::{DateTime, Utc, TimeZone, NaiveTime, Datelike, Timelike};
use chrono_tz::Tz;
use std::collections::HashMap;

/// Timezone Manager for seamless global recognition posting
/// 
/// Ensures all recognition posts are timed perfectly for maximum positive
/// engagement based on local timezone, cultural patterns, and optimal activity periods
#[derive(Clone)]
pub struct TimezoneManager {
    /// Optimal posting times by timezone and day type
    optimal_times: HashMap<String, OptimalTimeWindows>,
    
    /// Cultural timing preferences by region
    cultural_preferences: HashMap<String, CulturalTimingPrefs>,
}

#[derive(Debug, Clone)]
pub struct OptimalTimeWindows {
    pub weekday_morning: TimeWindow,
    pub weekday_lunch: TimeWindow, 
    pub weekday_evening: TimeWindow,
    pub weekend_morning: TimeWindow,
    pub weekend_afternoon: TimeWindow,
    pub weekend_evening: TimeWindow,
}

#[derive(Debug, Clone)]
pub struct TimeWindow {
    pub start_hour: u32,
    pub end_hour: u32,
    pub engagement_multiplier: f64,
    pub harmony_boost: f64,
}

#[derive(Debug, Clone)]
pub struct CulturalTimingPrefs {
    pub meal_times: Vec<TimeWindow>,
    pub prayer_times: Vec<TimeWindow>,
    pub work_hours: TimeWindow,
    pub family_time: TimeWindow,
    pub cultural_sensitivity_factor: f64,
}

impl TimezoneManager {
    pub fn new() -> Self {
        let optimal_times = Self::initialize_optimal_times();
        let cultural_preferences = Self::initialize_cultural_preferences();
        
        Self {
            optimal_times,
            cultural_preferences,
        }
    }

    /// Calculate the absolute optimal posting time for maximum positive engagement
    pub fn calculate_optimal_posting_time(
        &self, 
        timezone: &str,
        event_type: &str,
        target_demographics: &[String],
    ) -> anyhow::Result<DateTime<Utc>> {
        let tz: Tz = timezone.parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse timezone {}: {}", timezone, e))?;
        let now_local = Utc::now().with_timezone(&tz);
        
        // Get optimal time windows for this timezone
        let time_windows = self.optimal_times.get(timezone)
            .or_else(|| self.get_default_optimal_times_for_region(&tz))
            .ok_or_else(|| anyhow::anyhow!("No optimal times found for timezone: {}", timezone))?;
        
        // Select best time window based on event type and day
        let best_window = self.select_optimal_window(
            time_windows,
            event_type,
            &now_local,
            target_demographics,
        );
        
        // Calculate specific time within the window
        let optimal_time = self.calculate_precise_time_in_window(
            &best_window,
            &now_local,
            event_type,
            timezone,
        )?;
        
        Ok(optimal_time.with_timezone(&Utc))
    }

    /// Get all timezone posting times for global coordination
    pub fn calculate_global_posting_schedule(
        &self,
        event_type: &str,
        target_timezones: &[String],
    ) -> anyhow::Result<Vec<(String, DateTime<Utc>)>> {
        let mut schedule = Vec::new();
        
        for timezone in target_timezones {
            let optimal_time = self.calculate_optimal_posting_time(
                timezone,
                event_type,
                &["general".to_string()],
            )?;
            
            schedule.push((timezone.clone(), optimal_time));
        }
        
        // Sort by posting time to create a global wave of recognition
        schedule.sort_by_key(|(_, time)| *time);
        
        Ok(schedule)
    }

    /// Check if a specific time respects cultural sensitivities
    pub fn is_culturally_appropriate_time(
        &self,
        timezone: &str,
        posting_time: &DateTime<Utc>,
        event_type: &str,
    ) -> bool {
        let tz: Tz = match timezone.parse() {
            Ok(tz) => tz,
            Err(_) => return true, // Default to appropriate if timezone parsing fails
        };
        
        let local_time = posting_time.with_timezone(&tz);
        let hour = local_time.hour();
        let minute = local_time.minute();
        
        // Check cultural preferences for this region
        if let Some(prefs) = self.get_cultural_preferences_for_timezone(timezone) {
            // Avoid prayer times for Islamic regions
            for prayer_time in &prefs.prayer_times {
                if hour >= prayer_time.start_hour && hour <= prayer_time.end_hour {
                    return false;
                }
            }
            
            // Avoid meal times for family-oriented cultures
            for meal_time in &prefs.meal_times {
                if hour >= meal_time.start_hour && hour <= meal_time.end_hour {
                    return false;
                }
            }
            
            // Respect work hours for professional achievements
            if event_type.contains("career") || event_type.contains("work") {
                let work_hours = &prefs.work_hours;
                if hour < work_hours.start_hour || hour > work_hours.end_hour {
                    return false;
                }
            }
        }
        
        // General appropriateness checks
        self.is_generally_appropriate_time(hour, minute, event_type)
    }

    /// Calculate engagement probability at a specific time
    pub fn calculate_engagement_probability(
        &self,
        timezone: &str,
        posting_time: &DateTime<Utc>,
        event_type: &str,
    ) -> f64 {
        let tz: Tz = match timezone.parse() {
            Ok(tz) => tz,
            Err(_) => return 0.5, // Default moderate engagement
        };
        
        let local_time = posting_time.with_timezone(&tz);
        let hour = local_time.hour();
        let is_weekend = matches!(local_time.weekday(), chrono::Weekday::Sat | chrono::Weekday::Sun);
        
        // Base engagement by hour
        let base_engagement = match hour {
            6..=8 => 0.7,   // Early morning: moderate engagement
            9..=11 => 0.9,  // Morning: high engagement
            12..=13 => 0.8, // Lunch: good engagement
            14..=16 => 0.85, // Afternoon: good engagement
            17..=19 => 0.95, // Evening: peak engagement
            20..=22 => 0.8,  // Night: good engagement
            _ => 0.4,       // Late night/early morning: low engagement
        };
        
        // Weekend adjustment
        let weekend_multiplier = if is_weekend {
            match hour {
                10..=14 => 1.2, // Weekend mid-day peak
                15..=18 => 1.1, // Weekend afternoon
                _ => 0.9,       // Other weekend times slightly lower
            }
        } else {
            1.0
        };
        
        // Event type adjustment
        let event_multiplier = match event_type {
            t if t.contains("family") => if is_weekend { 1.2 } else { 0.8 },
            t if t.contains("work") || t.contains("career") => if is_weekend { 0.7 } else { 1.1 },
            t if t.contains("celebration") => 1.15,
            t if t.contains("achievement") => 1.1,
            _ => 1.0,
        };
        
        // Cultural sensitivity adjustment
        let cultural_multiplier = if self.is_culturally_appropriate_time(timezone, posting_time, event_type) {
            1.0
        } else {
            0.3 // Severely reduced engagement for culturally inappropriate times
        };
        
        (base_engagement * weekend_multiplier * event_multiplier * cultural_multiplier).min(1.0)
    }

    /// Initialize optimal posting times for different timezone groups
    fn initialize_optimal_times() -> HashMap<String, OptimalTimeWindows> {
        let mut times = HashMap::new();
        
        // North American timezones
        let na_windows = OptimalTimeWindows {
            weekday_morning: TimeWindow { start_hour: 9, end_hour: 11, engagement_multiplier: 0.9, harmony_boost: 1.1 },
            weekday_lunch: TimeWindow { start_hour: 12, end_hour: 14, engagement_multiplier: 0.8, harmony_boost: 1.0 },
            weekday_evening: TimeWindow { start_hour: 17, end_hour: 19, engagement_multiplier: 0.95, harmony_boost: 1.2 },
            weekend_morning: TimeWindow { start_hour: 10, end_hour: 12, engagement_multiplier: 0.85, harmony_boost: 1.15 },
            weekend_afternoon: TimeWindow { start_hour: 13, end_hour: 15, engagement_multiplier: 0.9, harmony_boost: 1.1 },
            weekend_evening: TimeWindow { start_hour: 18, end_hour: 20, engagement_multiplier: 0.8, harmony_boost: 1.0 },
        };
        
        times.insert("America/New_York".to_string(), na_windows.clone());
        times.insert("America/Chicago".to_string(), na_windows.clone());
        times.insert("America/Denver".to_string(), na_windows.clone());
        times.insert("America/Los_Angeles".to_string(), na_windows);
        
        // European timezones
        let eu_windows = OptimalTimeWindows {
            weekday_morning: TimeWindow { start_hour: 8, end_hour: 10, engagement_multiplier: 0.85, harmony_boost: 1.0 },
            weekday_lunch: TimeWindow { start_hour: 12, end_hour: 14, engagement_multiplier: 0.7, harmony_boost: 0.9 },
            weekday_evening: TimeWindow { start_hour: 18, end_hour: 20, engagement_multiplier: 0.9, harmony_boost: 1.1 },
            weekend_morning: TimeWindow { start_hour: 9, end_hour: 11, engagement_multiplier: 0.8, harmony_boost: 1.0 },
            weekend_afternoon: TimeWindow { start_hour: 14, end_hour: 16, engagement_multiplier: 0.85, harmony_boost: 1.05 },
            weekend_evening: TimeWindow { start_hour: 19, end_hour: 21, engagement_multiplier: 0.75, harmony_boost: 0.95 },
        };
        
        times.insert("Europe/London".to_string(), eu_windows.clone());
        times.insert("Europe/Paris".to_string(), eu_windows.clone());
        times.insert("Europe/Berlin".to_string(), eu_windows.clone());
        times.insert("Europe/Rome".to_string(), eu_windows);
        
        // Asian timezones
        let asia_windows = OptimalTimeWindows {
            weekday_morning: TimeWindow { start_hour: 8, end_hour: 10, engagement_multiplier: 0.9, harmony_boost: 1.15 },
            weekday_lunch: TimeWindow { start_hour: 12, end_hour: 13, engagement_multiplier: 0.6, harmony_boost: 0.8 },
            weekday_evening: TimeWindow { start_hour: 19, end_hour: 21, engagement_multiplier: 0.95, harmony_boost: 1.2 },
            weekend_morning: TimeWindow { start_hour: 9, end_hour: 11, engagement_multiplier: 0.8, harmony_boost: 1.1 },
            weekend_afternoon: TimeWindow { start_hour: 15, end_hour: 17, engagement_multiplier: 0.85, harmony_boost: 1.0 },
            weekend_evening: TimeWindow { start_hour: 20, end_hour: 22, engagement_multiplier: 0.9, harmony_boost: 1.05 },
        };
        
        times.insert("Asia/Tokyo".to_string(), asia_windows.clone());
        times.insert("Asia/Shanghai".to_string(), asia_windows.clone());
        times.insert("Asia/Seoul".to_string(), asia_windows.clone());
        times.insert("Asia/Hong_Kong".to_string(), asia_windows);
        
        times
    }

    /// Initialize cultural timing preferences
    fn initialize_cultural_preferences() -> HashMap<String, CulturalTimingPrefs> {
        let mut prefs = HashMap::new();
        
        // Islamic regions
        let islamic_prefs = CulturalTimingPrefs {
            meal_times: vec![
                TimeWindow { start_hour: 12, end_hour: 13, engagement_multiplier: 0.3, harmony_boost: 0.5 }, // Lunch
                TimeWindow { start_hour: 19, end_hour: 20, engagement_multiplier: 0.3, harmony_boost: 0.5 }, // Dinner
            ],
            prayer_times: vec![
                TimeWindow { start_hour: 5, end_hour: 6, engagement_multiplier: 0.1, harmony_boost: 0.2 }, // Fajr
                TimeWindow { start_hour: 12, end_hour: 13, engagement_multiplier: 0.1, harmony_boost: 0.2 }, // Dhuhr  
                TimeWindow { start_hour: 15, end_hour: 16, engagement_multiplier: 0.1, harmony_boost: 0.2 }, // Asr
                TimeWindow { start_hour: 18, end_hour: 19, engagement_multiplier: 0.1, harmony_boost: 0.2 }, // Maghrib
                TimeWindow { start_hour: 20, end_hour: 21, engagement_multiplier: 0.1, harmony_boost: 0.2 }, // Isha
            ],
            work_hours: TimeWindow { start_hour: 8, end_hour: 17, engagement_multiplier: 1.0, harmony_boost: 1.0 },
            family_time: TimeWindow { start_hour: 19, end_hour: 22, engagement_multiplier: 1.2, harmony_boost: 1.3 },
            cultural_sensitivity_factor: 1.5,
        };
        
        prefs.insert("Middle_East".to_string(), islamic_prefs);
        
        // Asian family-oriented cultures
        let asian_family_prefs = CulturalTimingPrefs {
            meal_times: vec![
                TimeWindow { start_hour: 12, end_hour: 13, engagement_multiplier: 0.4, harmony_boost: 0.6 },
                TimeWindow { start_hour: 18, end_hour: 19, engagement_multiplier: 0.3, harmony_boost: 0.5 },
            ],
            prayer_times: Vec::new(),
            work_hours: TimeWindow { start_hour: 9, end_hour: 18, engagement_multiplier: 1.0, harmony_boost: 1.0 },
            family_time: TimeWindow { start_hour: 19, end_hour: 22, engagement_multiplier: 1.3, harmony_boost: 1.4 },
            cultural_sensitivity_factor: 1.3,
        };
        
        prefs.insert("East_Asia".to_string(), asian_family_prefs);
        
        prefs
    }

    fn select_optimal_window<'a>(
        &self,
        windows: &'a OptimalTimeWindows,
        event_type: &str,
        local_now: &chrono::DateTime<Tz>,
        _target_demographics: &[String],
    ) -> &'a TimeWindow {
        let is_weekend = matches!(local_now.weekday(), chrono::Weekday::Sat | chrono::Weekday::Sun);
        
        if is_weekend {
            match event_type {
                t if t.contains("family") || t.contains("personal") => &windows.weekend_afternoon,
                t if t.contains("celebration") => &windows.weekend_evening,
                _ => &windows.weekend_morning,
            }
        } else {
            match event_type {
                t if t.contains("work") || t.contains("career") => &windows.weekday_morning,
                t if t.contains("achievement") => &windows.weekday_evening,
                _ => &windows.weekday_lunch,
            }
        }
    }

    fn calculate_precise_time_in_window(
        &self,
        window: &TimeWindow,
        local_now: &chrono::DateTime<Tz>,
        _event_type: &str,
        timezone: &str,
    ) -> anyhow::Result<chrono::DateTime<Tz>> {
        // Calculate optimal minute within the hour for maximum engagement
        let optimal_hour = window.start_hour + (window.end_hour - window.start_hour) / 2;
        
        // Use golden ratio for optimal minute selection
        let golden_ratio = (1.0 + 5.0_f64.sqrt()) / 2.0;
        let optimal_minute = ((golden_ratio * 60.0) % 60.0) as u32;
        
        // Create the optimal datetime
        let mut optimal_date = local_now.date_naive();
        
        // If the optimal time has already passed today, schedule for tomorrow
        let optimal_time = NaiveTime::from_hms_opt(optimal_hour, optimal_minute, 0)
            .ok_or_else(|| anyhow::anyhow!("Invalid time: {}:{}", optimal_hour, optimal_minute))?;
        
        if local_now.time() > optimal_time {
            optimal_date = optimal_date.succ_opt()
                .ok_or_else(|| anyhow::anyhow!("Failed to calculate next day"))?;
        }
        
        let naive_datetime = optimal_date.and_time(optimal_time);
        let tz: Tz = timezone.parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse timezone {}: {}", timezone, e))?;
        
        Ok(tz.from_local_datetime(&naive_datetime)
            .single()
            .ok_or_else(|| anyhow::anyhow!("Ambiguous local datetime"))?)
    }

    fn get_default_optimal_times_for_region(&self, tz: &Tz) -> Option<&OptimalTimeWindows> {
        // Map timezone to region and return appropriate default
        let tz_name = tz.name();
        if tz_name.starts_with("America/") {
            self.optimal_times.get("America/New_York")
        } else if tz_name.starts_with("Europe/") {
            self.optimal_times.get("Europe/London")
        } else if tz_name.starts_with("Asia/") {
            self.optimal_times.get("Asia/Tokyo")
        } else {
            None
        }
    }

    fn get_cultural_preferences_for_timezone(&self, timezone: &str) -> Option<&CulturalTimingPrefs> {
        // Map timezone to cultural region
        if timezone.contains("Middle_East") || timezone.contains("Asia/Dubai") || timezone.contains("Asia/Riyadh") {
            self.cultural_preferences.get("Middle_East")
        } else if timezone.starts_with("Asia/") {
            self.cultural_preferences.get("East_Asia")
        } else {
            None
        }
    }

    fn is_generally_appropriate_time(&self, hour: u32, _minute: u32, event_type: &str) -> bool {
        match event_type {
            t if t.contains("work") || t.contains("career") => (8..18).contains(&hour),
            t if t.contains("family") => (10..22).contains(&hour),
            t if t.contains("celebration") => (12..23).contains(&hour),
            _ => (7..23).contains(&hour), // General appropriate hours
        }
    }
}