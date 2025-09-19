-- Daily Recognition System Schema
-- Enhanced schema for global harmony and workforce-aligned recognition system

-- Daily recognitions table - core recognition posts
CREATE TABLE daily_recognitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    recognition_type VARCHAR(50) NOT NULL CHECK (recognition_type IN ('LocalDaily', 'StateWeekly', 'CountryWeekly', 'GlobalDiversity')),
    event_description TEXT NOT NULL,
    achievement_details TEXT NOT NULL,
    location JSONB NOT NULL, -- Store Location struct as JSON
    scheduled_post_time TIMESTAMPTZ NOT NULL,
    actual_post_time TIMESTAMPTZ,
    engagement_score DOUBLE PRECISION,
    harmony_contribution_score DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    industry_relevance VARCHAR(100),
    career_impact_score DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    status VARCHAR(20) NOT NULL DEFAULT 'Scheduled' CHECK (status IN ('Scheduled', 'Posted', 'Engaged', 'Completed', 'Skipped')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Workforce statistics cache for real-time industry analysis
CREATE TABLE workforce_statistics_cache (
    city VARCHAR(100) NOT NULL,
    radius_km DOUBLE PRECISION NOT NULL,
    total_employment BIGINT NOT NULL,
    industry_breakdown JSONB NOT NULL,
    labor_shortages JSONB NOT NULL,
    labor_surpluses JSONB NOT NULL,  
    growth_sectors JSONB NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (city, radius_km)
);

-- Employment data for workforce analysis calculations
CREATE TABLE employment_data (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    industry VARCHAR(100) NOT NULL,
    profession VARCHAR(100) NOT NULL,
    city VARCHAR(100) NOT NULL,
    state_province VARCHAR(100) NOT NULL,
    country VARCHAR(100) NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    worker_count INTEGER NOT NULL DEFAULT 1,
    salary DOUBLE PRECISION,
    skill_level VARCHAR(20) CHECK (skill_level IN ('Beginner', 'Intermediate', 'Advanced', 'Expert', 'Master')),
    employment_status VARCHAR(20) DEFAULT 'Active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- City coordinates for radius calculations
CREATE TABLE city_coordinates (
    city_name VARCHAR(100) PRIMARY KEY,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    country VARCHAR(100) NOT NULL,
    population BIGINT,
    timezone VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Job market data for demand/supply analysis
CREATE TABLE job_market_data (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    industry VARCHAR(100) NOT NULL,
    profession VARCHAR(100) NOT NULL,
    city VARCHAR(100) NOT NULL,
    type VARCHAR(20) NOT NULL CHECK (type IN ('posting', 'application')),
    salary_offered DOUBLE PRECISION,
    requirements TEXT[],
    urgency_level VARCHAR(20) CHECK (urgency_level IN ('Immediate', 'High', 'Moderate', 'Low')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Industry skill requirements mapping
CREATE TABLE industry_skill_requirements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    industry VARCHAR(100) NOT NULL,
    skill_name VARCHAR(100) NOT NULL,
    importance_score DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    skill_category VARCHAR(50),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(industry, skill_name)
);

-- Training programs availability
CREATE TABLE training_programs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    profession VARCHAR(100) NOT NULL,
    program_name VARCHAR(200) NOT NULL,
    provider VARCHAR(200) NOT NULL,
    duration_weeks INTEGER,
    cost_usd DOUBLE PRECISION,
    location VARCHAR(200),
    online_available BOOLEAN DEFAULT FALSE,
    active BOOLEAN DEFAULT TRUE,
    success_rate DOUBLE PRECISION,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Historical engagement data for machine learning
CREATE TABLE historical_engagement_data (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    recognition_id UUID REFERENCES daily_recognitions(id),
    posted_time TIMESTAMPTZ NOT NULL,
    timezone VARCHAR(50) NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    engagement_score DOUBLE PRECISION NOT NULL,
    positive_reactions INTEGER DEFAULT 0,
    negative_reactions INTEGER DEFAULT 0,
    total_reactions INTEGER DEFAULT 0,
    reach INTEGER DEFAULT 0,
    comments_count INTEGER DEFAULT 0,
    shares_count INTEGER DEFAULT 0,
    harmony_impact_score DOUBLE PRECISION DEFAULT 0.0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Engagement feedback for continuous model improvement
CREATE TABLE engagement_feedback (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    posted_time TIMESTAMPTZ NOT NULL,
    actual_engagement DOUBLE PRECISION NOT NULL,
    predicted_engagement DOUBLE PRECISION,
    event_type VARCHAR(100) NOT NULL,
    target_audience JSONB NOT NULL,
    accuracy_score DOUBLE PRECISION,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Global harmony metrics tracking
CREATE TABLE global_harmony_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    measurement_date DATE NOT NULL,
    region VARCHAR(100) NOT NULL,
    unity_score DOUBLE PRECISION NOT NULL,
    peace_contribution DOUBLE PRECISION NOT NULL,
    diversity_factor DOUBLE PRECISION NOT NULL,
    community_strength DOUBLE PRECISION NOT NULL,
    conflict_reduction DOUBLE PRECISION NOT NULL,
    global_impact DOUBLE PRECISION NOT NULL,
    total_harmony_score DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(measurement_date, region)
);

-- Recognition scheduling queue for automated posting
CREATE TABLE recognition_posting_queue (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    recognition_id UUID NOT NULL REFERENCES daily_recognitions(id),
    scheduled_time TIMESTAMPTZ NOT NULL,
    priority_score DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    processing_status VARCHAR(20) NOT NULL DEFAULT 'Queued' CHECK (processing_status IN ('Queued', 'Processing', 'Posted', 'Failed', 'Cancelled')),
    attempts INTEGER DEFAULT 0,
    last_attempt TIMESTAMPTZ,
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for performance optimization
CREATE INDEX idx_daily_recognitions_scheduled_time ON daily_recognitions(scheduled_post_time);
CREATE INDEX idx_daily_recognitions_location_city ON daily_recognitions USING GIN ((location->>'city'));
CREATE INDEX idx_daily_recognitions_status ON daily_recognitions(status);
CREATE INDEX idx_daily_recognitions_type ON daily_recognitions(recognition_type);
CREATE INDEX idx_daily_recognitions_harmony_score ON daily_recognitions(harmony_contribution_score DESC);

CREATE INDEX idx_employment_data_location ON employment_data(city, industry);
CREATE INDEX idx_employment_data_coords ON employment_data(latitude, longitude);
CREATE INDEX idx_employment_data_industry ON employment_data(industry);

CREATE INDEX idx_workforce_stats_city ON workforce_statistics_cache(city);
CREATE INDEX idx_workforce_stats_updated ON workforce_statistics_cache(updated_at);

CREATE INDEX idx_job_market_city_industry ON job_market_data(city, industry);
CREATE INDEX idx_job_market_type ON job_market_data(type);
CREATE INDEX idx_job_market_created ON job_market_data(created_at);

CREATE INDEX idx_historical_engagement_timezone ON historical_engagement_data(timezone);
CREATE INDEX idx_historical_engagement_time ON historical_engagement_data(posted_time);
CREATE INDEX idx_historical_engagement_type ON historical_engagement_data(event_type);

CREATE INDEX idx_city_coordinates_country ON city_coordinates(country);
CREATE INDEX idx_city_coordinates_coords ON city_coordinates(latitude, longitude);

CREATE INDEX idx_posting_queue_scheduled ON recognition_posting_queue(scheduled_time);
CREATE INDEX idx_posting_queue_status ON recognition_posting_queue(processing_status);

-- Triggers for automatic timestamp updates
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_daily_recognitions_updated_at BEFORE UPDATE ON daily_recognitions 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_employment_data_updated_at BEFORE UPDATE ON employment_data 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Views for common queries

-- View for active daily recognitions with location details
CREATE VIEW active_daily_recognitions AS
SELECT 
    dr.*,
    (dr.location->>'city') as city,
    (dr.location->>'state_province') as state_province,
    (dr.location->>'country') as country,
    (dr.location->>'timezone') as timezone
FROM daily_recognitions dr
WHERE dr.status IN ('Scheduled', 'Posted', 'Engaged');

-- View for workforce shortage analysis
CREATE VIEW workforce_shortage_analysis AS
SELECT 
    ed.city,
    ed.industry,
    ed.profession,
    COUNT(*) as current_workers,
    COALESCE(jmd_postings.posting_count, 0) as job_postings,
    COALESCE(jmd_applications.application_count, 0) as applications,
    CASE 
        WHEN COALESCE(jmd_applications.application_count, 0) > 0 
        THEN COALESCE(jmd_postings.posting_count, 0)::FLOAT / jmd_applications.application_count
        ELSE 0
    END as demand_ratio
FROM employment_data ed
LEFT JOIN (
    SELECT city, industry, profession, COUNT(*) as posting_count
    FROM job_market_data 
    WHERE type = 'posting' AND created_at > NOW() - INTERVAL '30 days'
    GROUP BY city, industry, profession
) jmd_postings ON ed.city = jmd_postings.city 
    AND ed.industry = jmd_postings.industry 
    AND ed.profession = jmd_postings.profession
LEFT JOIN (
    SELECT city, industry, profession, COUNT(*) as application_count
    FROM job_market_data 
    WHERE type = 'application' AND created_at > NOW() - INTERVAL '30 days'
    GROUP BY city, industry, profession
) jmd_applications ON ed.city = jmd_applications.city 
    AND ed.industry = jmd_applications.industry 
    AND ed.profession = jmd_applications.profession
WHERE ed.employment_status = 'Active'
GROUP BY ed.city, ed.industry, ed.profession, jmd_postings.posting_count, jmd_applications.application_count;

-- View for optimal posting times analysis
CREATE VIEW optimal_posting_times AS
SELECT 
    timezone,
    event_type,
    EXTRACT(hour FROM posted_time) as hour,
    EXTRACT(dow FROM posted_time) as day_of_week,
    AVG(engagement_score) as avg_engagement,
    AVG(harmony_impact_score) as avg_harmony_impact,
    COUNT(*) as sample_size
FROM historical_engagement_data
WHERE created_at > NOW() - INTERVAL '90 days'
GROUP BY timezone, event_type, EXTRACT(hour FROM posted_time), EXTRACT(dow FROM posted_time)
HAVING COUNT(*) >= 5
ORDER BY avg_engagement DESC, avg_harmony_impact DESC;

-- Materialized view for global harmony dashboard
CREATE MATERIALIZED VIEW global_harmony_dashboard AS
SELECT 
    measurement_date,
    AVG(unity_score) as global_unity_score,
    AVG(peace_contribution) as global_peace_score,
    AVG(diversity_factor) as global_diversity_score,
    AVG(community_strength) as global_community_score,
    AVG(conflict_reduction) as global_conflict_reduction,
    AVG(total_harmony_score) as overall_global_harmony,
    COUNT(DISTINCT region) as regions_measured
FROM global_harmony_metrics
GROUP BY measurement_date
ORDER BY measurement_date DESC;

-- Refresh the materialized view daily
CREATE INDEX idx_global_harmony_dashboard_date ON global_harmony_dashboard(measurement_date);

-- Grant permissions for application user
-- GRANT SELECT, INSERT, UPDATE ON ALL TABLES IN SCHEMA public TO app_user;
-- GRANT USAGE ON ALL SEQUENCES IN SCHEMA public TO app_user;

-- Sample data for testing (remove in production)
INSERT INTO city_coordinates (city_name, latitude, longitude, country, population, timezone) VALUES
('New York', 40.7128, -74.0060, 'United States', 8336817, 'America/New_York'),
('London', 51.5074, -0.1278, 'United Kingdom', 9648110, 'Europe/London'),
('Tokyo', 35.6762, 139.6503, 'Japan', 14094034, 'Asia/Tokyo'),
('Sydney', -33.8688, 151.2093, 'Australia', 5312163, 'Australia/Sydney'),
('Berlin', 52.5200, 13.4050, 'Germany', 3669491, 'Europe/Berlin'),
('São Paulo', -23.5505, -46.6333, 'Brazil', 12325232, 'America/Sao_Paulo');

-- Sample industry skill requirements
INSERT INTO industry_skill_requirements (industry, skill_name, importance_score, skill_category) VALUES
('technology', 'Python Programming', 0.9, 'Technical'),
('technology', 'JavaScript', 0.85, 'Technical'),
('technology', 'Cloud Computing', 0.8, 'Technical'),
('healthcare', 'Patient Care', 0.95, 'Clinical'),
('healthcare', 'Medical Knowledge', 0.9, 'Clinical'),
('education', 'Teaching Methods', 0.9, 'Pedagogical'),
('education', 'Curriculum Development', 0.75, 'Pedagogical'),
('construction', 'Safety Protocols', 0.95, 'Safety'),
('construction', 'Blueprint Reading', 0.8, 'Technical');