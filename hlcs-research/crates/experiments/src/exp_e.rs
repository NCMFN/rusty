use forex_sim::{simulate_burst, BurstResult};
use hlcs_core::{hybrid_commit, PublicParams};
use rand::{rngs::StdRng, SeedableRng};
use std::time::Instant;

pub fn run(
    seed: u64,
    target_orders_per_sec: u64,
    window_ms: f64,
    n: usize,
    q: u32,
    sigma: f64,
) -> BurstResult {
    let mut rng = StdRng::seed_from_u64(seed);
    let pp = PublicParams::generate(n, q, &mut rng);
    let msg = b"burst order payload";

    // Measure average single-threaded latency to process one order
    let warmup = 1000;
    for _ in 0..warmup {
        let _ = hybrid_commit(&pp, msg, sigma, &mut rng);
    }

    let trials = 10_000;
    let start = Instant::now();
    for _ in 0..trials {
        let _ = hybrid_commit(&pp, msg, sigma, &mut rng);
    }
    let total_ns = start.elapsed().as_nanos() as f64;
    let per_order_latency_ns = total_ns / (trials as f64);

    simulate_burst(target_orders_per_sec, window_ms, per_order_latency_ns)
}
