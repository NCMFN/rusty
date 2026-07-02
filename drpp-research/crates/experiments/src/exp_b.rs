use crate::config::Config;
use drpp_core::adversary::collusion_attack;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rayon::prelude::*;

#[derive(Debug)]
pub struct ResultB {
    pub k: u32,
    pub n_colluders: usize,
    pub p_theoretical: f64,
    pub p_simulated: f64,
}

pub fn run(cfg: &Config) -> anyhow::Result<Vec<ResultB>> {
    let mut tasks = Vec::new();
    for &n in &cfg.colluders {
        for k in 1..=cfg.k_max_coll {
            tasks.push((n, k));
        }
    }

    let results: Vec<ResultB> = tasks
        .into_par_iter()
        .map(|(n, k)| {
            let mut rng = StdRng::seed_from_u64(cfg.seed + (n * 100) as u64 + k as u64);
            let mut wins = 0;

            for _ in 0..cfg.n_trials {
                if collusion_attack(k, n, cfg.prf_secret.as_bytes(), &mut rng) {
                    wins += 1;
                }
            }

            let p_simulated = wins as f64 / cfg.n_trials as f64;

            let space = 1u64 << k;
            let p_theoretical = if n as u64 >= space {
                1.0
            } else {
                n as f64 / space as f64
            };

            ResultB {
                k,
                n_colluders: n,
                p_theoretical,
                p_simulated,
            }
        })
        .collect();

    Ok(results)
}
