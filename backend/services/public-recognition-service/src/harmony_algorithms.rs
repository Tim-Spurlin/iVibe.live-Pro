use crate::models::*;
use nalgebra::{DVector, DMatrix};
use std::f64::consts::PI;

/// Advanced Harmony Algorithms for Global Peace and Unity
/// 
/// This module implements sophisticated mathematical formulas derived from
/// ancient wisdom traditions, modern social psychology, and systems theory
/// to calculate harmony scores and promote global peace.
#[derive(Clone)]
pub struct HarmonyAlgorithms {
    // Golden ratio for optimal harmony calculations
    golden_ratio: f64,
    // Fibonacci sequence for natural harmony patterns
    fibonacci_sequence: Vec<u64>,
    // Cultural harmony weights
    cultural_weights: DMatrix<f64>,
}

impl HarmonyAlgorithms {
    pub fn new() -> Self {
        let golden_ratio = (1.0 + 5.0_f64.sqrt()) / 2.0;
        let fibonacci_sequence = vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610];
        
        // Initialize cultural harmony weights matrix (simplified 5x5)
        // Represents: Western, Eastern, African, Indigenous, Middle Eastern cultures
        let cultural_weights = DMatrix::from_row_slice(5, 5, &[
            1.0, 0.8, 0.7, 0.6, 0.7,  // Western harmony with others
            0.8, 1.0, 0.9, 0.8, 0.6,  // Eastern harmony with others
            0.7, 0.9, 1.0, 0.9, 0.8,  // African harmony with others
            0.6, 0.8, 0.9, 1.0, 0.7,  // Indigenous harmony with others
            0.7, 0.6, 0.8, 0.7, 1.0,  // Middle Eastern harmony with others
        ]);

        Self {
            golden_ratio,
            fibonacci_sequence,
            cultural_weights,
        }
    }

    /// Calculate Global Harmony Score using the Universal Peace Formula
    /// 
    /// Formula: H = Σ(i=1 to n) [φ^i × (U_i × P_i × D_i × C_i × T_i)] × Ω(location) × Ψ(time)
    /// 
    /// Where:
    /// - φ (phi) = Golden Ratio (1.618...)
    /// - U_i = Unity factor for element i
    /// - P_i = Peace contribution factor
    /// - D_i = Diversity promotion factor  
    /// - C_i = Community building factor
    /// - T_i = Trust enhancement factor
    /// - Ω(location) = Geographic harmony amplifier
    /// - Ψ(time) = Temporal harmony modifier
    pub async fn calculate_global_harmony_score(
        &self,
        event_type: &str,
        location: &Location,
        context: &HarmonyContext,
    ) -> f64 {
        // Initialize base harmony components
        let unity_score = self.calculate_unity_factor(event_type, context).await;
        let peace_score = self.calculate_peace_contribution(event_type, context).await;
        let diversity_score = self.calculate_diversity_factor(location, context).await;
        let community_score = self.calculate_community_building_factor(event_type, context).await;
        let trust_score = self.calculate_trust_enhancement_factor(context).await;

        // Apply golden ratio weighting to create natural harmony
        let weighted_components = DVector::from_vec(vec![
            unity_score * self.golden_ratio.powi(1),
            peace_score * self.golden_ratio.powi(2),
            diversity_score * self.golden_ratio.powi(3),
            community_score * self.golden_ratio.powi(4),
            trust_score * self.golden_ratio.powi(5),
        ]);

        // Calculate base harmony score
        let base_score = weighted_components.sum() / weighted_components.len() as f64;

        // Apply geographic harmony amplifier
        let geographic_amplifier = self.calculate_geographic_harmony_amplifier(location).await;
        
        // Apply temporal harmony modifier
        let temporal_modifier = self.calculate_temporal_harmony_modifier(&context.timing_factors).await;

        // Final harmony score with all modifiers
        let final_score = base_score * geographic_amplifier * temporal_modifier;

        // Normalize to 0-1 range and apply sigmoid for smooth distribution
        self.apply_harmony_sigmoid(final_score)
    }

    /// Calculate Unity Factor - How much this event promotes human unity
    async fn calculate_unity_factor(&self, event_type: &str, context: &HarmonyContext) -> f64 {
        let mut unity_score = 0.0;

        // Base unity scores for different event types
        unity_score += match event_type {
            t if t.contains("wedding") || t.contains("marriage") => 0.7,
            t if t.contains("birth") || t.contains("baby") => 0.8,
            t if t.contains("graduation") => 0.6,
            t if t.contains("volunteer") || t.contains("charity") => 0.9,
            t if t.contains("community service") => 0.85,
            t if t.contains("helping") || t.contains("assistance") => 0.8,
            t if t.contains("teaching") || t.contains("mentoring") => 0.75,
            t if t.contains("cultural exchange") => 0.9,
            t if t.contains("peace") || t.contains("reconciliation") => 0.95,
            t if t.contains("cooperation") || t.contains("collaboration") => 0.8,
            _ => 0.5, // Default neutral score
        };

        // Add social factors contribution
        for social_factor in &context.social_factors {
            if social_factor.positive_impact {
                unity_score += social_factor.weight * 0.1;
            } else {
                unity_score -= social_factor.weight * 0.05;
            }
        }

        // Apply cultural diversity bonus
        let cultural_diversity_bonus = self.calculate_cultural_diversity_bonus(context).await;
        unity_score += cultural_diversity_bonus;

        // Ensure score is in valid range
        unity_score.max(0.0).min(1.0)
    }

    /// Calculate Peace Contribution - How much this promotes peace and reduces conflict
    async fn calculate_peace_contribution(&self, event_type: &str, context: &HarmonyContext) -> f64 {
        let mut peace_score = 0.0;

        // Base peace contribution scores
        peace_score += match event_type {
            t if t.contains("conflict resolution") => 0.95,
            t if t.contains("mediation") => 0.9,
            t if t.contains("apology") || t.contains("forgiveness") => 0.85,
            t if t.contains("dialogue") || t.contains("discussion") => 0.7,
            t if t.contains("understanding") => 0.75,
            t if t.contains("empathy") || t.contains("compassion") => 0.8,
            t if t.contains("healing") || t.contains("recovery") => 0.7,
            t if t.contains("justice") || t.contains("fairness") => 0.75,
            t if t.contains("equality") || t.contains("inclusion") => 0.8,
            _ => 0.4, // Default moderate peace contribution
        };

        // Economic factors that promote peace through prosperity
        for economic_factor in &context.economic_factors {
            if economic_factor.job_creation_potential > 0 {
                // Job creation reduces poverty and conflict
                peace_score += (economic_factor.job_creation_potential as f64 / 1000.0) * 0.1;
            }
            
            if economic_factor.economic_impact > 0.0 {
                // Positive economic impact promotes stability
                peace_score += economic_factor.economic_impact * 0.05;
            }
        }

        // Apply conflict reduction multiplier
        let conflict_reduction_bonus = self.calculate_conflict_reduction_bonus(context).await;
        peace_score *= (1.0 + conflict_reduction_bonus);

        peace_score.max(0.0).min(1.0)
    }

    /// Calculate Diversity Factor - How much this celebrates and promotes diversity
    async fn calculate_diversity_factor(&self, location: &Location, context: &HarmonyContext) -> f64 {
        let mut diversity_score = 0.0;

        // Base diversity score from location demographics
        diversity_score += self.get_location_diversity_index(location).await;

        // Cultural factors contribution
        for cultural_factor in &context.cultural_factors {
            if cultural_factor.diversity_promotion {
                diversity_score += cultural_factor.cultural_sensitivity * 0.2;
            }
        }

        // Cross-cultural interaction bonus
        diversity_score += self.calculate_cross_cultural_interaction_bonus(context).await;

        // Language diversity bonus
        diversity_score += self.calculate_language_diversity_bonus(context).await;

        diversity_score.max(0.0).min(1.0)
    }

    /// Calculate Community Building Factor
    async fn calculate_community_building_factor(&self, event_type: &str, context: &HarmonyContext) -> f64 {
        let mut community_score = 0.0;

        // Base community building scores
        community_score += match event_type {
            t if t.contains("community") => 0.9,
            t if t.contains("neighborhood") => 0.8,
            t if t.contains("local") => 0.7,
            t if t.contains("group") || t.contains("team") => 0.75,
            t if t.contains("organization") => 0.6,
            t if t.contains("network") => 0.65,
            t if t.contains("support") => 0.8,
            t if t.contains("connection") => 0.75,
            _ => 0.3,
        };

        // Social network effect - stronger communities amplify harmony
        let network_effect = self.calculate_social_network_effect(context).await;
        community_score *= (1.0 + network_effect);

        community_score.max(0.0).min(1.0)
    }

    /// Calculate Trust Enhancement Factor
    async fn calculate_trust_enhancement_factor(&self, context: &HarmonyContext) -> f64 {
        let mut trust_score = 0.5; // Start with neutral trust

        // Social factors that build trust
        for social_factor in &context.social_factors {
            if social_factor.factor_type.contains("honesty") ||
               social_factor.factor_type.contains("transparency") ||
               social_factor.factor_type.contains("reliability") {
                trust_score += social_factor.weight * 0.15;
            }
        }

        // Economic stability builds trust
        let economic_stability = self.calculate_economic_stability_index(context).await;
        trust_score += economic_stability * 0.2;

        // Cultural respect builds trust across communities
        let cultural_respect = self.calculate_cultural_respect_index(context).await;
        trust_score += cultural_respect * 0.15;

        trust_score.max(0.0).min(1.0)
    }

    /// Calculate Geographic Harmony Amplifier using sacred geometry principles
    async fn calculate_geographic_harmony_amplifier(&self, location: &Location) -> f64 {
        // Apply latitude-based harmony modifier (closer to equator = higher base harmony)
        let latitude_factor = 1.0 - (location.latitude.abs() / 90.0) * 0.2;
        
        // Population density harmony (moderate density is optimal)
        let population_harmony = if let Some(pop) = location.population {
            match pop {
                0..=50_000 => 0.9,       // Small communities: high harmony
                50_001..=500_000 => 1.0, // Medium cities: optimal harmony
                500_001..=2_000_000 => 0.95, // Large cities: good harmony
                _ => 0.85,               // Megacities: reduced harmony due to stress
            }
        } else {
            0.9 // Default for unknown population
        };

        // Fibonacci spiral harmony (locations that align with natural patterns)
        let spiral_harmony = self.calculate_fibonacci_spiral_harmony(location).await;

        // Combine all geographic factors
        latitude_factor * population_harmony * spiral_harmony
    }

    /// Calculate Temporal Harmony Modifier using circadian and lunar cycles
    async fn calculate_temporal_harmony_modifier(&self, timing: &TimingFactors) -> f64 {
        let mut temporal_score = 1.0;

        // Day of week harmony (based on global energy patterns)
        temporal_score *= match timing.day_of_week.as_str() {
            "Monday" => 0.9,    // Fresh start energy
            "Tuesday" => 0.95,  // Building momentum
            "Wednesday" => 1.0, // Peak week energy
            "Thursday" => 0.95, // Sustained energy
            "Friday" => 0.85,   // Winding down (work focus shifts)
            "Saturday" => 0.9,  // Community time
            "Sunday" => 0.95,   // Reflection and connection
            _ => 1.0,
        };

        // Time of day harmony (based on circadian rhythms)
        temporal_score *= match timing.time_of_day.as_str() {
            "morning" => 1.1,   // High alertness and positivity
            "midday" => 1.0,    // Peak energy
            "afternoon" => 0.95, // Slight energy dip
            "evening" => 0.9,   // Relaxation time
            "night" => 0.8,     // Rest time (lower harmony for activities)
            _ => 1.0,
        };

        // Seasonal harmony bonus
        for season in &timing.seasonal_considerations {
            temporal_score *= match season.as_str() {
                "spring" => 1.1,   // Growth and renewal
                "summer" => 1.05,  // High energy and activity
                "autumn" => 1.0,   // Harvest and reflection
                "winter" => 0.95,  // Rest and planning
                _ => 1.0,
            };
        }

        // Cultural calendar harmony
        for event in &timing.cultural_calendar_events {
            if event.contains("celebration") || event.contains("festival") {
                temporal_score *= 1.15; // Celebrations amplify harmony
            } else if event.contains("memorial") || event.contains("remembrance") {
                temporal_score *= 1.05; // Respectful remembrance promotes unity
            }
        }

        temporal_score
    }

    /// Apply harmony sigmoid function for smooth distribution
    fn apply_harmony_sigmoid(&self, raw_score: f64) -> f64 {
        // Modified sigmoid function: f(x) = 2 / (1 + e^(-2x)) - 1
        // This maps any real number to (-1, 1), then we shift to (0, 1)
        let sigmoid = 2.0 / (1.0 + (-2.0 * raw_score).exp()) - 1.0;
        (sigmoid + 1.0) / 2.0 // Shift to (0, 1) range
    }

    // Helper methods for specific calculations

    async fn calculate_cultural_diversity_bonus(&self, context: &HarmonyContext) -> f64 {
        let cultural_count = context.cultural_factors.len() as f64;
        // More cultural factors = higher diversity bonus (up to 0.2)
        (cultural_count / 10.0).min(0.2)
    }

    async fn calculate_conflict_reduction_bonus(&self, context: &HarmonyContext) -> f64 {
        let mut bonus = 0.0;
        for social_factor in &context.social_factors {
            if social_factor.factor_type.contains("mediation") ||
               social_factor.factor_type.contains("resolution") ||
               social_factor.factor_type.contains("harmony") {
                bonus += social_factor.weight * 0.1;
            }
        }
        bonus.min(0.3) // Cap at 30% bonus
    }

    async fn get_location_diversity_index(&self, location: &Location) -> f64 {
        // This would typically query a diversity database
        // For now, return based on country diversity index
        match location.country.as_str() {
            "United States" => 0.8,
            "Canada" => 0.85,
            "Brazil" => 0.9,
            "India" => 0.95,
            "South Africa" => 0.9,
            "Australia" => 0.8,
            "United Kingdom" => 0.75,
            "Germany" => 0.7,
            "France" => 0.7,
            "Japan" => 0.6,
            _ => 0.65, // Default moderate diversity
        }
    }

    async fn calculate_cross_cultural_interaction_bonus(&self, context: &HarmonyContext) -> f64 {
        // Check if multiple cultures are represented
        let unique_cultures: std::collections::HashSet<_> = context.cultural_factors
            .iter()
            .map(|f| &f.aspect)
            .collect();
        
        match unique_cultures.len() {
            0..=1 => 0.0,
            2 => 0.1,
            3 => 0.15,
            4 => 0.2,
            _ => 0.25, // Maximum cross-cultural bonus
        }
    }

    async fn calculate_language_diversity_bonus(&self, _context: &HarmonyContext) -> f64 {
        // This would analyze language diversity in the event
        // For now, return a modest bonus
        0.05
    }

    async fn calculate_social_network_effect(&self, context: &HarmonyContext) -> f64 {
        // Network effect based on number of positive social factors
        let positive_factors = context.social_factors
            .iter()
            .filter(|f| f.positive_impact)
            .count() as f64;
        
        // Logarithmic scaling to prevent excessive network effects
        (positive_factors.ln() / 5.0).min(0.3)
    }

    async fn calculate_economic_stability_index(&self, context: &HarmonyContext) -> f64 {
        let mut stability = 0.0;
        let mut factor_count = 0;

        for economic_factor in &context.economic_factors {
            stability += economic_factor.economic_impact.abs().min(1.0);
            factor_count += 1;
        }

        if factor_count > 0 {
            stability / factor_count as f64
        } else {
            0.5 // Neutral stability
        }
    }

    async fn calculate_cultural_respect_index(&self, context: &HarmonyContext) -> f64 {
        let mut respect_sum = 0.0;
        let mut count = 0;

        for cultural_factor in &context.cultural_factors {
            respect_sum += cultural_factor.cultural_sensitivity;
            count += 1;
        }

        if count > 0 {
            respect_sum / count as f64
        } else {
            0.5 // Neutral respect
        }
    }

    async fn calculate_fibonacci_spiral_harmony(&self, location: &Location) -> f64 {
        // Calculate if location coordinates align with Fibonacci spiral patterns
        // This is a simplified version - real implementation would use complex geometric calculations
        
        let lat_fib = self.fibonacci_alignment(location.latitude);
        let lon_fib = self.fibonacci_alignment(location.longitude);
        
        // Combine latitude and longitude alignment
        (lat_fib + lon_fib) / 2.0
    }

    fn fibonacci_alignment(&self, coordinate: f64) -> f64 {
        // Check alignment with Fibonacci ratios
        let normalized = coordinate.abs() % 360.0; // Normalize to 0-360 range
        
        for &fib_num in &self.fibonacci_sequence {
            let fib_angle = (fib_num as f64 * self.golden_ratio) % 360.0;
            let alignment = 1.0 - (normalized - fib_angle).abs() / 180.0;
            if alignment > 0.8 {
                return 1.1; // Strong Fibonacci alignment bonus
            }
        }
        
        1.0 // No special alignment
    }
}

/// Peace Propagation Formula - calculates how harmony spreads through communities
/// 
/// P(t) = P₀ × e^(αt) × (1 - β × conflicts) × γ × diversity_factor
/// 
/// Where:
/// - P₀ = Initial peace level
/// - α = Growth rate constant
/// - β = Conflict dampening factor
/// - γ = Community resilience factor
/// - t = Time
pub fn calculate_peace_propagation(
    initial_peace: f64,
    time_elapsed: f64,
    conflict_level: f64,
    community_resilience: f64,
    diversity_factor: f64,
) -> f64 {
    let growth_rate = 0.1; // 10% base growth rate
    let conflict_dampening = 0.3; // 30% dampening per unit conflict
    
    let exponential_growth = initial_peace * (growth_rate * time_elapsed).exp();
    let conflict_reduction = 1.0 - (conflict_dampening * conflict_level);
    
    exponential_growth * conflict_reduction * community_resilience * diversity_factor
}

/// Global Unity Index - measures worldwide harmony levels
/// 
/// GUI = Σ(i=1 to n) [w_i × H_i × √(population_i)] / Σ(√(population_i))
/// 
/// Where:
/// - w_i = Weight for region i
/// - H_i = Harmony score for region i  
/// - population_i = Population of region i
pub fn calculate_global_unity_index(regional_data: &[(f64, f64, u64)]) -> f64 {
    let mut weighted_sum = 0.0;
    let mut weight_sum = 0.0;
    
    for &(weight, harmony, population) in regional_data {
        let pop_sqrt = (population as f64).sqrt();
        weighted_sum += weight * harmony * pop_sqrt;
        weight_sum += pop_sqrt;
    }
    
    if weight_sum > 0.0 {
        weighted_sum / weight_sum
    } else {
        0.0
    }
}