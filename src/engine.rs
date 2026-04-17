use std::sync::mpsc::Sender;

use rand::{RngExt, rngs::ThreadRng};

use crate::models::{BattleResult, ForceConfig, SimulationResult, WargameScenario};

pub fn run(config: &WargameScenario, sender: Option<Sender<BattleResult>>) -> SimulationResult {
    let mut rng = rand::rng();

    let iterations = config.simulation.iterations;
    let mut q_ratios = Vec::with_capacity(iterations);
    let mut success_count = 0;

    for i in 0..iterations {
        // Calculate Red Force Power
        let red_power = self::calculate_force_power(&config.force_red, &mut rng);
        let m_red = rng.random_range(config.force_red.m_min..=config.force_red.m_max);

        let final_red = red_power * m_red;

        // Calculate Blue Force Power
        let blue_power = self::calculate_force_power(&config.force_blue, &mut rng);
        let m_blue = rng.random_range(config.force_blue.m_min..=config.force_blue.m_max);

        let final_blue = blue_power * m_blue;

        // Resolve the COFM Ratio (Q)
        // Prevent division by zero in case of extreme edge case configurations
        let q_ratio = if final_blue > 0.0 {
            let raw_ratio = final_red / final_blue;
            // Round to 2 decimal places for optimized histogram binning
            (raw_ratio * 100.0).round() / 100.0
        } else {
            f32::MAX
        };

        // Push cofm result
        q_ratios.push(q_ratio);

        // Determine success
        let success = if q_ratio > config.simulation.success_threshold {
            success_count += 1;
            true
        } else {
            false
        };

        // Push to sender if available
        if let Some(s) = &sender {
            _ = s.send(BattleResult {
                iteration: i,
                red_env: m_red,
                red_quality: red_power,
                blue_env: m_blue,
                blue_quality: blue_power,
                red_total_force: final_red,
                blue_total_force: final_blue,
                cofm_ratio: q_ratio,
                win_side: if success {
                    "RED".to_string()
                } else {
                    "BLUE".to_string()
                },
            });
        }
    }

    let success_probability = (success_count as f32 / iterations as f32) * 100.0;

    SimulationResult {
        total_iterations: iterations,
        success_count,
        success_probability,
        q_ratios,
    }
}

/// Helper function to calculate the stochastic power of a single force
fn calculate_force_power(force: &ForceConfig, rng: &mut ThreadRng) -> f32 {
    let mut total_power = 0.0;

    for unit in &force.elements {
        // Determine the upper and lower bounds based on intelligence uncertainty (k_variance)
        let k_min = unit.base_quality * (1.0 - force.k_variance);
        let k_max = unit.base_quality * (1.0 + force.k_variance);

        // Randomize the unit's quality for this specific iteration
        let randomized_k = rng.random_range(k_min..=k_max);

        total_power += (unit.quantity as f32) * randomized_k;
    }

    total_power
}
