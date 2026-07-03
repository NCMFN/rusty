use hlcs_core::{hash_commit, hybrid_commit, lattice_commit, PublicParams};
use rand::{rngs::StdRng, SeedableRng};
use std::time::Instant;

pub struct ExpCDataPoint {
    pub n: usize,
    pub order_load: usize,
    pub hash_mean_ms: f64,
    pub lattice_mean_ms: f64,
    pub hybrid_mean_ms: f64,
}

pub struct ExpCResult {
    pub data: Vec<ExpCDataPoint>,
}

pub fn run(
    seed: u64,
    dimensions: &[usize],
    loads: &[usize],
    q: u32,
    sigma: f64,
    trials_per_point: usize,
) -> ExpCResult {
    let mut data = Vec::new();

    for &n in dimensions {
        let mut rng = StdRng::seed_from_u64(seed);
        let pp = PublicParams::generate(n, q, &mut rng);
        let message = b"buy 100 EURUSD";

        for &load in loads {
            let mut hash_latencies = Vec::with_capacity(trials_per_point);
            let mut lattice_latencies = Vec::with_capacity(trials_per_point);
            let mut hybrid_latencies = Vec::with_capacity(trials_per_point);

            for _ in 0..trials_per_point {
                // To simulate 'load' we could process a batch sequentially,
                // but the paper just shows "latency heatmap under load".
                // Here we'll simulate processing `load` orders sequentially and get the mean per order.

                let start = Instant::now();
                for _ in 0..load {
                    let _ = hash_commit(message, &mut rng);
                }
                hash_latencies
                    .push((start.elapsed().as_nanos() as f64 / 1_000_000.0) / load as f64);

                let start = Instant::now();
                for _ in 0..load {
                    let _ = lattice_commit(&pp, message, sigma, &mut rng);
                }
                lattice_latencies
                    .push((start.elapsed().as_nanos() as f64 / 1_000_000.0) / load as f64);

                let start = Instant::now();
                for _ in 0..load {
                    let _ = hybrid_commit(&pp, message, sigma, &mut rng);
                }
                hybrid_latencies
                    .push((start.elapsed().as_nanos() as f64 / 1_000_000.0) / load as f64);
            }

            let hash_mean_ms = hash_latencies.iter().sum::<f64>() / trials_per_point as f64;
            let lattice_mean_ms = lattice_latencies.iter().sum::<f64>() / trials_per_point as f64;
            let hybrid_mean_ms = hybrid_latencies.iter().sum::<f64>() / trials_per_point as f64;

            data.push(ExpCDataPoint {
                n,
                order_load: load,
                hash_mean_ms,
                lattice_mean_ms,
                hybrid_mean_ms,
            });
        }
    }

    ExpCResult { data }
}
