use crate::config::Config;
use drpp_core::adversary::single_guess_attack;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rayon::prelude::*;

#[derive(Debug)]
pub struct ResultF {
    pub k: u32,
    pub single_modal: f64,
    pub dual_modal: f64,
    pub triple_modal: f64,
}

pub fn run(cfg: &Config) -> anyhow::Result<Vec<ResultF>> {
    let results: Vec<ResultF> = (1..=cfg.k_max_drpp)
        .into_par_iter()
        .map(|k| {
            let mut rng = StdRng::seed_from_u64(cfg.seed + k as u64 * 10);

            let mut wins_single = 0;
            let mut wins_dual = 0;
            let mut wins_triple = 0;

            for _ in 0..cfg.n_trials {
                // A combined attack means the adversary has to succeed in all required modalities (AND combination).
                // This assumes modalities are independent challenge-response instances.

                let w1 = single_guess_attack(k, cfg.prf_secret.as_bytes(), &mut rng);
                let w2 = single_guess_attack(k, cfg.prf_secret.as_bytes(), &mut rng);
                let w3 = single_guess_attack(k, cfg.prf_secret.as_bytes(), &mut rng);

                if w1 {
                    wins_single += 1;
                }
                if w1 && w2 {
                    wins_dual += 1;
                }
                if w1 && w2 && w3 {
                    wins_triple += 1;
                }
            }

            ResultF {
                k,
                single_modal: wins_single as f64 / cfg.n_trials as f64,
                dual_modal: wins_dual as f64 / cfg.n_trials as f64,
                triple_modal: wins_triple as f64 / cfg.n_trials as f64,
            }
        })
        .collect();

    Ok(results)
}
