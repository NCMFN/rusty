use hlcs_core::{hash_commit, hybrid_commit, lattice_commit, PublicParams};
use rand::{rngs::StdRng, SeedableRng};
use std::time::Instant;

pub struct ExpAResult {
    pub hash_latencies_us: Vec<f64>,
    pub lattice_latencies_us: Vec<f64>,
    pub hybrid_latencies_us: Vec<f64>,
}

pub fn run(seed: u64, n_trials: usize, n: usize, q: u32, sigma: f64) -> ExpAResult {
    let mut rng = StdRng::seed_from_u64(seed);
    let pp = PublicParams::generate(n, q, &mut rng);
    let message = b"32-byte fixed message for exp A.."; // 32 bytes

    let mut hash_latencies_us = Vec::with_capacity(n_trials);
    let mut lattice_latencies_us = Vec::with_capacity(n_trials);
    let mut hybrid_latencies_us = Vec::with_capacity(n_trials);

    // Warmup
    for _ in 0..100 {
        let _ = hash_commit(message, &mut rng);
        let _ = lattice_commit(&pp, message, sigma, &mut rng);
        let _ = hybrid_commit(&pp, message, sigma, &mut rng);
    }

    for _ in 0..n_trials {
        let start = Instant::now();
        let _ = hash_commit(message, &mut rng);
        hash_latencies_us.push(start.elapsed().as_nanos() as f64 / 1000.0);

        let start = Instant::now();
        let _ = lattice_commit(&pp, message, sigma, &mut rng);
        lattice_latencies_us.push(start.elapsed().as_nanos() as f64 / 1000.0);

        let start = Instant::now();
        let _ = hybrid_commit(&pp, message, sigma, &mut rng);
        hybrid_latencies_us.push(start.elapsed().as_nanos() as f64 / 1000.0);
    }

    ExpAResult {
        hash_latencies_us,
        lattice_latencies_us,
        hybrid_latencies_us,
    }
}
