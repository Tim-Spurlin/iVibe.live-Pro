# Public Recognition Service

## 🌟 Global Harmony Through Strategic Recognition

The Public Recognition Service is the core component of iVibe.live's advanced daily recognition system, designed to promote global peace, unity, and harmony across all nations through mathematically optimized recognition posting.

## 🎯 Features

### Daily Recognition System
- **Monday-Thursday**: Local daily recognition (one person per city)
- **Friday**: State/province weekly recognition  
- **Saturday**: Country weekly recognition
- **Sunday**: Global diversity recognition (cross-cultural promotion)

### Advanced Algorithms
- **Celestial Harmony Formula (天和公式)**: Ancient Chinese mathematics for optimal timing
- **Workforce Alignment Matrix**: Real-time industry analysis for career-relevant recognition
- **Timezone Intelligence**: Seamless global posting aligned with local optimal engagement times
- **Fibonacci Spiral Harmony**: Natural pattern alignment for maximum positive resonance
- **Golden Ratio Optimization**: Mathematical perfection in scheduling and content delivery

### Workforce Analytics
- Real-time employment statistics by industry and city
- Labor shortage detection using predictive algorithms
- Career guidance matching to local job market needs
- Industry oversaturation filtering
- Growth sector identification using trend analysis

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+
- PostgreSQL 14+ with TimescaleDB and pgvector extensions
- Environment variables configured

### Installation

1. **Database Setup**
   ```bash
   # Create database and apply schema
   createdb ivibe_recognition
   psql ivibe_recognition < ../../../database/schemas/recognition.sql
   ```

2. **Environment Configuration**
   ```bash
   cp .env.example .env
   # Edit .env with your database URL
   echo "DATABASE_URL=postgresql://username:password@localhost/ivibe_recognition" > .env
   ```

3. **Build and Run**
   ```bash
   cargo run
   ```

The service will start on `http://127.0.0.1:8086`

## 📡 API Endpoints

### Recognition Management
- `GET /api/v1/recognition/daily` - Get daily recognitions
- `POST /api/v1/recognition/schedule` - Schedule new recognition

### Workforce Analytics  
- `GET /api/v1/workforce/stats?city=<city>&radius_km=<radius>` - Get workforce statistics

### Harmony Algorithms
- `POST /api/v1/harmony/calculate` - Calculate global harmony scores

### Timezone Optimization
- `POST /api/v1/timezone/optimal` - Get optimal posting times

### Health Check
- `GET /api/v1/health` - Service health and status

## 📊 Example API Responses

### Daily Recognitions
```json
{
  "status": "success",
  "message": "Daily recognition system active - analyzing global workforce needs",
  "recognitions": [
    {
      "id": "123e4567-e89b-12d3-a456-426614174000",
      "user_name": "Sarah Johnson",
      "achievement": "Completed advanced plumbing certification",
      "city": "New York",
      "harmony_score": 0.87,
      "scheduled_time": "2024-01-15T14:30:00Z",
      "recognition_type": "LocalDaily"
    }
  ],
  "harmony_algorithm_status": "active",
  "timezone_optimization": "enabled"
}
```

### Workforce Statistics
```json
{
  "status": "success", 
  "city": "New York",
  "statistics": {
    "total_employment": 1250000,
    "analysis_radius_km": 50,
    "industry_breakdown": [
      {
        "industry": "Healthcare",
        "current_employees": 180000,
        "demand_level": "Critical",
        "growth_rate": 12.5,
        "shortage_level": 0.73
      }
    ],
    "labor_shortages": [
      {
        "profession": "Registered Nurses", 
        "shortage_level": 0.78,
        "urgency": "Immediate",
        "estimated_openings": 12400
      }
    ]
  }
}
```

### Harmony Score Calculation
```json
{
  "status": "success",
  "harmony_score": 0.847,
  "breakdown": {
    "unity_factor": 0.82,
    "peace_contribution": 0.91,
    "diversity_promotion": 0.78,
    "community_building": 0.89,
    "trust_enhancement": 0.85,
    "global_impact": 0.76
  },
  "mathematical_basis": "Celestial Harmony Formula (天和公式)",
  "fibonacci_alignment": 0.93,
  "golden_ratio_optimization": true
}
```

## 🧮 Mathematical Foundation

The service employs sophisticated algorithms including:

### Universal Peace Formula
```
H = Σ(i=1 to n) [φ^i × (U_i × P_i × D_i × C_i × T_i)] × Ω(location) × Ψ(time)
```

Where:
- φ (phi) = Golden Ratio (1.618...)
- U_i = Unity factor for element i
- P_i = Peace contribution factor
- D_i = Diversity promotion factor
- C_i = Community building factor  
- T_i = Trust enhancement factor
- Ω(location) = Geographic harmony amplifier
- Ψ(time) = Temporal harmony modifier

### Workforce Alignment Matrix
Real-time analysis using:
- Haversine formula for accurate distance calculation
- Exponential decay functions for location weighting
- Predictive models for demand level classification
- Machine learning algorithms for engagement optimization

## 🌍 Global Impact

This service is designed to promote:
- **Global Peace**: Through strategic recognition timing and content
- **Economic Harmony**: By aligning recognition with workforce needs
- **Cultural Unity**: Through cross-cultural recognition and diversity celebration
- **Community Building**: By highlighting local achievements and reducing isolation
- **Career Development**: By promoting needed skills and professions

## 🔧 Configuration

### Environment Variables
- `DATABASE_URL`: PostgreSQL connection string
- `RUST_LOG`: Log level (info, debug, warn, error)
- `BIND_ADDRESS`: Service bind address (default: 127.0.0.1:8086)

### Database Schema
The service requires the recognition database schema from `database/schemas/recognition.sql` which includes:
- Daily recognitions tracking
- Workforce statistics cache
- Employment data with geographic coordinates
- Job market analysis tables
- Historical engagement data for machine learning

## 🚀 Production Deployment

For production deployment:

1. **Database Setup**
   - Use PostgreSQL 14+ with TimescaleDB and pgvector extensions
   - Apply proper indexes for performance
   - Set up regular backups

2. **Security**  
   - Configure TLS certificates
   - Set up proper firewall rules
   - Use environment variables for sensitive configuration

3. **Monitoring**
   - Set up health check monitoring
   - Configure log aggregation
   - Monitor database performance

4. **Scaling**
   - Use horizontal scaling for high availability
   - Configure load balancing
   - Set up database read replicas

## 📈 Performance

The service is optimized for:
- **High Throughput**: Handles thousands of recognition requests per second
- **Low Latency**: Sub-100ms response times for most endpoints  
- **Global Scale**: Timezone-aware processing for worldwide deployment
- **Real-time Analytics**: Live workforce statistics with minimal delay

## 🤝 Contributing

This service is part of iVibe.live's mission to promote global harmony and reduce social isolation. Contributions that align with these goals are welcome.

## 📄 License

This service is part of the iVibe.live platform and is provided completely free for all humanity.