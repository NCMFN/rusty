use hlcs_core::{hybrid_commit, hybrid_verify, lattice_commit, lattice_verify, PublicParams};
use rand::{rngs::StdRng, SeedableRng};
use std::time::Instant;

pub struct CommOverheadPoint {
    pub scheme: String,
    pub bytes: usize,
    pub security_bits: u32,
}

pub struct AblationResult {
    pub short_circuit_verify_latencies_us: Vec<f64>,
    pub full_lattice_verify_latencies_us: Vec<f64>,
}

pub struct ExpKResult {
    pub comm_data: Vec<CommOverheadPoint>,
    pub ablation: AblationResult,
}

pub fn run(seed: u64, n_trials: usize, n: usize, q: u32, sigma: f64) -> ExpKResult {
    let mut rng = StdRng::seed_from_u64(seed);
    let pp = PublicParams::generate(n, q, &mut rng);
    let msg = b"ablation and comm overhead";

    // Comm overhead bytes
    // Hash: 32 bytes commit, 32 bytes r + msg (opening)
    // Lattice: n * 4 bytes commit, n * 4 bytes r + n * 8 bytes e + msg (opening)
    // Hybrid: 32 + n*4 bytes commit, n*4 + n*8 bytes + msg (opening)

    let hash_bytes = 32; // just C
    let lattice_bytes = n * 4;
    let hybrid_bytes = 32 + n * 4;

    let mut comm_data = Vec::new();
    comm_data.push(CommOverheadPoint {
        scheme: "Hash-only".into(),
        bytes: hash_bytes,
        security_bits: 256,
    });
    comm_data.push(CommOverheadPoint {
        scheme: "Lattice-only".into(),
        bytes: lattice_bytes,
        security_bits: 512,
    });
    comm_data.push(CommOverheadPoint {
        scheme: "Hybrid".into(),
        bytes: hybrid_bytes,
        security_bits: 512,
    });

    // Ablation
    let mut short_circuit_verify_latencies_us = Vec::with_capacity(n_trials);
    let mut full_lattice_verify_latencies_us = Vec::with_capacity(n_trials);

    for _ in 0..100 {
        let (commit, opening) = hybrid_commit(&pp, msg, sigma, &mut rng);
        let mut tampered_commit = commit.clone();
        tampered_commit.c1[0] ^= 1; // force hash check to fail
        let _ = hybrid_verify(&pp, &tampered_commit, &opening);

        let (lat_commit, lat_opening) = lattice_commit(&pp, msg, sigma, &mut rng);
        let mut tampered_lat_commit = lat_commit.clone();
        tampered_lat_commit.c2[0] = (tampered_lat_commit.c2[0] + 1) % pp.q;
        let _ = lattice_verify(&pp, &tampered_lat_commit, &lat_opening);
    }

    for _ in 0..n_trials {
        let (commit, opening) = hybrid_commit(&pp, msg, sigma, &mut rng);
        let mut tampered_commit = commit.clone();
        tampered_commit.c1[0] ^= 1; // force hash check to fail

        let start = Instant::now();
        let _ = hybrid_verify(&pp, &tampered_commit, &opening);
        short_circuit_verify_latencies_us.push(start.elapsed().as_nanos() as f64 / 1000.0);

        let (lat_commit, lat_opening) = lattice_commit(&pp, msg, sigma, &mut rng);
        let mut tampered_lat_commit = lat_commit.clone();
        tampered_lat_commit.c2[0] = (tampered_lat_commit.c2[0] + 1) % pp.q;

        let start = Instant::now();
        let _ = lattice_verify(&pp, &tampered_lat_commit, &lat_opening);
        full_lattice_verify_latencies_us.push(start.elapsed().as_nanos() as f64 / 1000.0);
    }

    ExpKResult {
        comm_data,
        ablation: AblationResult {
            short_circuit_verify_latencies_us,
            full_lattice_verify_latencies_us,
        },
    }
}
