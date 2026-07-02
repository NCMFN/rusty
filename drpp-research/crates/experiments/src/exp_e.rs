use crate::config::Config;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_distr::{Distribution, LogNormal};
use rayon::prelude::*;

#[derive(Debug)]
pub struct ResultE {
    pub modality: String,
    pub latencies: Vec<f64>,
    pub mean: f64,
    pub median: f64,
    pub std: f64,
    pub p95: f64,
}

pub fn run(cfg: &Config) -> anyhow::Result<Vec<ResultE>> {
    let modalities = vec![
        ("knock", 5.0, 0.5),   // e^5 ms ~ 148ms
        ("touch", 4.0, 0.4),   // e^4 ms ~ 54ms
        ("gesture", 5.5, 0.6), // e^5.5 ms ~ 244ms
    ];

    let results: Vec<ResultE> = modalities
        .into_par_iter()
        .map(|(modality, mu, sigma)| {
            let mut rng = StdRng::seed_from_u64(cfg.seed + modality.len() as u64);
            let dist = LogNormal::new(mu, sigma).unwrap();

            let mut latencies = Vec::with_capacity(cfg.latency_samples);
            for _ in 0..cfg.latency_samples {
                latencies.push(dist.sample(&mut rng) / 1000.0); // Convert to seconds
            }

            latencies.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let sum: f64 = latencies.iter().sum();
            let mean = sum / cfg.latency_samples as f64;

            let mut var_sum = 0.0;
            for &l in &latencies {
                var_sum += (l - mean).powi(2);
            }
            let std = (var_sum / cfg.latency_samples as f64).sqrt();

            let median = latencies[cfg.latency_samples / 2];
            let p95_idx = (cfg.latency_samples as f64 * 0.95) as usize;
            let p95 = latencies[p95_idx];

            ResultE {
                modality: modality.to_string(),
                latencies,
                mean,
                median,
                std,
                p95,
            }
        })
        .collect();

    Ok(results)
}
