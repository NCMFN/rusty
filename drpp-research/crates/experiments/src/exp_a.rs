use crate::config::Config;
use drpp_core::adversary::single_guess_attack;
use drpp_core::stats::ci_95;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rayon::prelude::*;

#[derive(Debug)]
pub struct ResultA {
    pub k: u32,
    pub p_theoretical: f64,
    pub p_simulated: f64,
    pub ci_lo: f64,
    pub ci_hi: f64,
    pub n_trials: u64,
}

pub fn run(cfg: &Config) -> anyhow::Result<Vec<ResultA>> {
    let results: Vec<ResultA> = (1..=cfg.k_max_drpp)
        .into_par_iter()
        .map(|k| {
            let mut rng = StdRng::seed_from_u64(cfg.seed + k as u64);
            let mut wins = 0;

            for _ in 0..cfg.n_trials {
                if single_guess_attack(k, cfg.prf_secret.as_bytes(), &mut rng) {
                    wins += 1;
                }
            }

            let p_simulated = wins as f64 / cfg.n_trials as f64;
            let p_theoretical = 1.0 / (1u64 << k) as f64;
            let (ci_lo, ci_hi) = ci_95(wins, cfg.n_trials);

            ResultA {
                k,
                p_theoretical,
                p_simulated,
                ci_lo,
                ci_hi,
                n_trials: cfg.n_trials,
            }
        })
        .collect();

    Ok(results)
}
