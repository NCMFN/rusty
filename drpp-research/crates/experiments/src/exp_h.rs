use crate::config::Config;
use drpp_core::adversary::single_guess_attack;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct ResultH {
    pub n_trials: u64,
    pub p_simulated: f64,
    pub p_theoretical: f64,
}

pub fn run(cfg: &Config) -> anyhow::Result<Vec<ResultH>> {
    let k = cfg.convergence_k;
    let p_theoretical = 1.0 / (1u64 << k) as f64;

    let results: Vec<ResultH> = cfg
        .convergence_trials
        .par_iter()
        .map(|&trials| {
            let mut rng = StdRng::seed_from_u64(cfg.seed + trials);
            let mut wins = 0;

            for _ in 0..trials {
                if single_guess_attack(k, cfg.prf_secret.as_bytes(), &mut rng) {
                    wins += 1;
                }
            }

            ResultH {
                n_trials: trials,
                p_simulated: wins as f64 / trials as f64,
                p_theoretical,
            }
        })
        .collect();

    Ok(results)
}
