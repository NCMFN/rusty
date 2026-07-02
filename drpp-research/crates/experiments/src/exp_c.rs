use crate::config::Config;
use drpp_core::adversary::traditional_attack;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rayon::prelude::*;

#[derive(Debug)]
pub struct ResultC {
    pub deception_prob: f64,
    pub p_simulated: f64,
    pub n_trials: u64,
}

pub fn run(cfg: &Config) -> anyhow::Result<Vec<ResultC>> {
    let results: Vec<ResultC> = cfg
        .deception_probs
        .par_iter()
        .map(|&dp| {
            let mut rng = StdRng::seed_from_u64(cfg.seed + (dp * 1000.0) as u64);
            let mut wins = 0;

            for _ in 0..cfg.n_trials {
                if traditional_attack(dp, &mut rng) {
                    wins += 1;
                }
            }

            let p_simulated = wins as f64 / cfg.n_trials as f64;

            ResultC {
                deception_prob: dp,
                p_simulated,
                n_trials: cfg.n_trials,
            }
        })
        .collect();

    Ok(results)
}
