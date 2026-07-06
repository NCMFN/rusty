use hlcs_core::{calculate_stats, hybrid_commit, zk_prove, zk_verify, LatencyStats, PublicParams};
use rand::{rngs::StdRng, SeedableRng};
use std::time::Instant;

pub struct ExpIResult {
    pub prove_time_us: LatencyStats,
    pub verify_time_us: LatencyStats,
    pub completeness_rate: f64,
    pub soundness_violation_rate: f64,
    pub n_trials: usize,
}

pub fn run(seed: u64, n_trials: usize, n: usize, q: u32, sigma: f64) -> ExpIResult {
    let mut rng = StdRng::seed_from_u64(seed);
    let pp = PublicParams::generate(n, q, &mut rng);
    let msg = b"zk proof test message";
    let sigma_prime = 10.0; // larger sigma for ZK

    let mut prove_times = Vec::with_capacity(n_trials);
    let mut verify_times = Vec::with_capacity(n_trials);

    let mut completeness_success = 0;

    for _ in 0..n_trials {
        let (commit, opening) = hybrid_commit(&pp, msg, sigma, &mut rng);

        let start = Instant::now();
        let proof = zk_prove(
            &pp,
            &commit,
            msg,
            &opening.r,
            &opening.e,
            sigma_prime,
            &mut rng,
        );
        prove_times.push(start.elapsed().as_nanos() as f64 / 1000.0);

        let start = Instant::now();
        let ok = zk_verify(&pp, &commit, &proof, msg);
        verify_times.push(start.elapsed().as_nanos() as f64 / 1000.0);

        if ok {
            completeness_success += 1;
        }
    }

    let mut soundness_violations = 0;
    for _ in 0..n_trials {
        let (commit, mut opening) = hybrid_commit(&pp, msg, sigma, &mut rng);

        // Malicious witness: tamper with r
        opening.r[0] = (opening.r[0] + 1) % pp.q;

        // The malicious prover tries to construct a proof using the tampered witness
        let proof = zk_prove(
            &pp,
            &commit,
            msg,
            &opening.r,
            &opening.e,
            sigma_prime,
            &mut rng,
        );

        let ok = zk_verify(&pp, &commit, &proof, msg);
        if ok {
            soundness_violations += 1;
        }
    }

    ExpIResult {
        prove_time_us: calculate_stats(prove_times),
        verify_time_us: calculate_stats(verify_times),
        completeness_rate: completeness_success as f64 / n_trials as f64,
        soundness_violation_rate: soundness_violations as f64 / n_trials as f64,
        n_trials,
    }
}
