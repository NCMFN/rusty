use hlcs_core::{hash_commit, hybrid_commit, lattice_commit, PublicParams};
use rand::{rngs::StdRng, SeedableRng};
use rayon::prelude::*;
use std::time::Instant;

pub struct ExpBResult {
    pub hash_tps_single: f64,
    pub hash_tps_multi: f64,
    pub lattice_tps_single: f64,
    pub lattice_tps_multi: f64,
    pub hybrid_tps_single: f64,
    pub hybrid_tps_multi: f64,
}

pub fn run(seed: u64, n_trials: usize, n: usize, q: u32, sigma: f64) -> ExpBResult {
    let mut rng = StdRng::seed_from_u64(seed);
    let pp = PublicParams::generate(n, q, &mut rng);
    let message = b"32-byte fixed message for exp B..";

    // Single-threaded hash
    let start = Instant::now();
    for _ in 0..n_trials {
        let mut local_rng = StdRng::seed_from_u64(seed); // deterministic for speed test
        let _ = hash_commit(message, &mut local_rng);
    }
    let duration = start.elapsed().as_secs_f64();
    let hash_tps_single = n_trials as f64 / duration;

    // Single-threaded lattice
    let start = Instant::now();
    for _ in 0..n_trials {
        let mut local_rng = StdRng::seed_from_u64(seed);
        let _ = lattice_commit(&pp, message, sigma, &mut local_rng);
    }
    let duration = start.elapsed().as_secs_f64();
    let lattice_tps_single = n_trials as f64 / duration;

    // Single-threaded hybrid
    let start = Instant::now();
    for _ in 0..n_trials {
        let mut local_rng = StdRng::seed_from_u64(seed);
        let _ = hybrid_commit(&pp, message, sigma, &mut local_rng);
    }
    let duration = start.elapsed().as_secs_f64();
    let hybrid_tps_single = n_trials as f64 / duration;

    // Multi-threaded
    let seeds: Vec<u64> = (0..n_trials as u64).collect();

    // Multi-threaded hash
    let start = Instant::now();
    seeds.par_iter().for_each(|&s| {
        let mut local_rng = StdRng::seed_from_u64(s);
        let _ = hash_commit(message, &mut local_rng);
    });
    let duration = start.elapsed().as_secs_f64();
    let hash_tps_multi = n_trials as f64 / duration;

    // Multi-threaded lattice
    let start = Instant::now();
    seeds.par_iter().for_each(|&s| {
        let mut local_rng = StdRng::seed_from_u64(s);
        let _ = lattice_commit(&pp, message, sigma, &mut local_rng);
    });
    let duration = start.elapsed().as_secs_f64();
    let lattice_tps_multi = n_trials as f64 / duration;

    // Multi-threaded hybrid
    let start = Instant::now();
    seeds.par_iter().for_each(|&s| {
        let mut local_rng = StdRng::seed_from_u64(s);
        let _ = hybrid_commit(&pp, message, sigma, &mut local_rng);
    });
    let duration = start.elapsed().as_secs_f64();
    let hybrid_tps_multi = n_trials as f64 / duration;

    ExpBResult {
        hash_tps_single,
        hash_tps_multi,
        lattice_tps_single,
        lattice_tps_multi,
        hybrid_tps_single,
        hybrid_tps_multi,
    }
}
