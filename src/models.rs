use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct WargameScenario {
    pub simulation: SimulationMeta,
    pub force_red: ForceConfig,
    pub force_blue: ForceConfig,
}

#[derive(Debug, Deserialize)]
pub struct SimulationMeta {
    pub iterations: usize,
    pub description: String,
    pub success_threshold: f32,
}

#[derive(Debug, Deserialize)]
pub struct ForceConfig {
    pub name: String,
    pub m_min: f32,
    pub m_max: f32,
    pub k_variance: f32,
    #[serde(rename = "units")]
    pub elements: Vec<UnitConfig>,
}

#[derive(Debug, Deserialize)]
pub struct UnitConfig {
    pub name: String,
    pub quantity: u32,
    pub base_quality: f32,
}

pub struct SimulationResult {
    pub total_iterations: usize,
    pub success_count: usize,
    pub success_probability: f32,
    pub q_ratios: Vec<f32>,
}

#[derive(Debug, Serialize)]
pub struct BattleResult {
    pub iteration: usize,
    pub red_power: f32,
    pub blue_power: f32,
    pub cofm_ratio: f32,
    pub win_side: String,
}
