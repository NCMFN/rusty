use anyhow::Result;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub seed: u64,
    pub n_trials: u64,
    pub k_max_drpp: u32,
    pub k_max_coll: u32,
    pub colluders: Vec<usize>,
    pub noise_levels: Vec<f64>,
    pub deception_probs: Vec<f64>,
    pub modality_samples: usize,
    pub latency_samples: usize,
    pub convergence_k: u32,
    pub convergence_trials: Vec<u64>,
    pub prf_secret: String,
    pub figure_width_px: u32,
    pub figure_height_px: u32,
}

pub fn load_config(path: &str) -> Result<Config> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
