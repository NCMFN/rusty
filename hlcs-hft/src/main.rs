use csv::Writer;
use rand::rngs::StdRng;
use rand::SeedableRng;

use std::hint::black_box;
use std::path::Path;
use std::time::Instant;

use hlcs_hft::*;

const MSG: &[u8] = b"buy 1000 BTC @ 50000";

fn main() {
    println!("Starting HLCS-HFT benchmarks...");
    let dims = vec![128, 256, 512, 768, 1024];

    bench_latency_throughput(&dims);
    bench_bandwidth(&dims);
    bench_overload(&dims);
    bench_zk(&dims);
}

fn bench_latency_throughput(dims: &[usize]) {
    println!("Running latency/throughput benchmarks...");
    let path = Path::new("bench_results/latency_throughput.csv");
    let mut wtr = Writer::from_path(path).unwrap();
    wtr.write_record(["n", "scheme", "mean_latency_ms", "ops_per_sec"])
        .unwrap();

    let mut rng = StdRng::seed_from_u64(42);
    let trials = 2000;

    for &n in dims {
        let pp = PublicParams {
            a: generate_matrix(n, &mut rng),
            n,
        };

        // 1. Hash-only
        let start = Instant::now();
        for _ in 0..trials {
            let (c, hint) = commit_hash_only(MSG, n, &mut rng);
            black_box(verify_hash_only(&c, &hint));
        }
        let elapsed = start.elapsed();
        let latency_ms = elapsed.as_secs_f64() * 1000.0 / trials as f64;
        let ops = trials as f64 / elapsed.as_secs_f64();
        wtr.write_record([
            n.to_string(),
            "hash_only".to_string(),
            latency_ms.to_string(),
            ops.to_string(),
        ])
        .unwrap();

        // 2. Lattice-only
        let mut buf_ar = vec![0; n];
        let mut buf_c2 = vec![0; n];
        let start = Instant::now();
        for _ in 0..trials {
            let (c, hint) = commit_lattice_only(&pp, MSG, &mut rng, &mut buf_ar, &mut buf_c2);
            black_box(verify_lattice_only(&pp, &c, &hint));
        }
        let elapsed = start.elapsed();
        let latency_ms = elapsed.as_secs_f64() * 1000.0 / trials as f64;
        let ops = trials as f64 / elapsed.as_secs_f64();
        wtr.write_record([
            n.to_string(),
            "lattice_only".to_string(),
            latency_ms.to_string(),
            ops.to_string(),
        ])
        .unwrap();

        // 3. Hybrid
        let mut buf_ar = vec![0; n];
        let mut buf_c2 = vec![0; n];
        let start = Instant::now();
        for _ in 0..trials {
            let (c, hint) = commit_hybrid(&pp, MSG, &mut rng, &mut buf_ar, &mut buf_c2);
            black_box(verify_hybrid(&pp, &c, &hint));
        }
        let elapsed = start.elapsed();
        let latency_ms = elapsed.as_secs_f64() * 1000.0 / trials as f64;
        let ops = trials as f64 / elapsed.as_secs_f64();
        wtr.write_record([
            n.to_string(),
            "hybrid".to_string(),
            latency_ms.to_string(),
            ops.to_string(),
        ])
        .unwrap();
    }
    wtr.flush().unwrap();
}

fn bench_bandwidth(dims: &[usize]) {
    println!("Running bandwidth benchmarks...");
    let path = Path::new("bench_results/bandwidth.csv");
    let mut wtr = Writer::from_path(path).unwrap();
    wtr.write_record(["n", "scheme", "commit_size_bytes", "bw_per_1000_kb"])
        .unwrap();

    let mut rng = StdRng::seed_from_u64(42);

    for &n in dims {
        let pp = PublicParams {
            a: generate_matrix(n, &mut rng),
            n,
        };
        let mut buf_ar = vec![0; n];
        let mut buf_c2 = vec![0; n];

        // Size calculation: c1 is 32 bytes. c2 is vec of i64 (8 bytes each).

        let (c_hash, _) = commit_hash_only(MSG, n, &mut rng);
        let size_hash = c_hash.c1.len();
        let bw_hash = (size_hash * 1000) as f64 / 1024.0;
        wtr.write_record([
            n.to_string(),
            "hash_only".to_string(),
            size_hash.to_string(),
            bw_hash.to_string(),
        ])
        .unwrap();

        let (c_lat, _) = commit_lattice_only(&pp, MSG, &mut rng, &mut buf_ar, &mut buf_c2);
        let size_lat = c_lat.c2.len() * 8; // i64 is 8 bytes
        let bw_lat = (size_lat * 1000) as f64 / 1024.0;
        wtr.write_record([
            n.to_string(),
            "lattice_only".to_string(),
            size_lat.to_string(),
            bw_lat.to_string(),
        ])
        .unwrap();

        let (c_hyb, _) = commit_hybrid(&pp, MSG, &mut rng, &mut buf_ar, &mut buf_c2);
        let size_hyb = c_hyb.c1.len() + c_hyb.c2.len() * 8;
        let bw_hyb = (size_hyb * 1000) as f64 / 1024.0;
        wtr.write_record([
            n.to_string(),
            "hybrid".to_string(),
            size_hyb.to_string(),
            bw_hyb.to_string(),
        ])
        .unwrap();
    }
    wtr.flush().unwrap();
}

fn bench_overload(dims: &[usize]) {
    println!("Running overload p99 benchmarks...");
    let path = Path::new("bench_results/overload_p99.csv");
    let mut wtr = Writer::from_path(path).unwrap();
    wtr.write_record(["n", "load_rate", "p99_latency_ms", "jitter", "sla_breach"])
        .unwrap();

    let mut rng = StdRng::seed_from_u64(42);
    let loads = vec![100, 500, 1000, 5000, 10000, 20000];

    for &n in dims {
        let pp = PublicParams {
            a: generate_matrix(n, &mut rng),
            n,
        };

        for &load in &loads {
            let mut latencies = Vec::with_capacity(2000);
            let mut buf_ar = vec![0; n];
            let mut buf_c2 = vec![0; n];

            for _ in 0..2000 {
                let start = Instant::now();
                let (c, hint) = commit_hybrid(&pp, MSG, &mut rng, &mut buf_ar, &mut buf_c2);
                black_box(verify_hybrid(&pp, &c, &hint));
                latencies.push(start.elapsed().as_secs_f64() * 1000.0);
            }

            latencies.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let p99 = latencies[(latencies.len() as f64 * 0.99) as usize];
            let mean = latencies.iter().sum::<f64>() / latencies.len() as f64;
            let variance = latencies
                .iter()
                .map(|value| {
                    let diff = mean - *value;
                    diff * diff
                })
                .sum::<f64>()
                / latencies.len() as f64;
            let jitter = variance.sqrt() / mean;

            let _breach = if p99 > 1.0 { "true" } else { "false" };

            // Apply simulated queuing delay based on Little's Law / M/M/1
            let capacity = 1000.0 / mean; // ops per sec max capacity
            let rho = load as f64 / capacity;

            let simulated_p99 = if rho >= 1.0 {
                p99 * 100.0 // Overload
            } else {
                p99 / (1.0 - rho)
            };

            let sim_breach = if simulated_p99 > 1.0 { "true" } else { "false" };

            wtr.write_record([
                n.to_string(),
                load.to_string(),
                simulated_p99.to_string(),
                jitter.to_string(),
                sim_breach.to_string(),
            ])
            .unwrap();
        }
    }
    wtr.flush().unwrap();
}

fn bench_zk(dims: &[usize]) {
    println!("Running ZK proof benchmarks...");
    let path = Path::new("bench_results/zk_proof.csv");
    let mut wtr = Writer::from_path(path).unwrap();
    wtr.write_record(["n", "rounds", "time_ms", "size_bytes"])
        .unwrap();

    let mut rng = StdRng::seed_from_u64(42);
    let trials = 100;

    for &n in dims {
        let pp = PublicParams {
            a: generate_matrix(n, &mut rng),
            n,
        };
        let mut buf_ar = vec![0; n];
        let mut buf_c2 = vec![0; n];

        let (c, hint) = commit_hybrid(&pp, MSG, &mut rng, &mut buf_ar, &mut buf_c2);

        // 1-round
        let start = Instant::now();
        let mut proof_size = 0;
        for _ in 0..trials {
            let proof = prove_zk_1round(&pp, &c, &hint, &mut rng);
            black_box(verify_zk_1round(&pp, &c, MSG, &proof));
            proof_size = proof.t1.len() * 8 + proof.sr.len() * 8 + proof.se.len() * 8;
        }
        let elapsed = start.elapsed();
        let time_ms = elapsed.as_secs_f64() * 1000.0 / trials as f64;

        wtr.write_record([
            n.to_string(),
            "1".to_string(),
            time_ms.to_string(),
            proof_size.to_string(),
        ])
        .unwrap();

        // 10-round
        let start = Instant::now();
        let mut proof_size = 0;
        for _ in 0..trials {
            let proofs = prove_zk_10round(&pp, &c, &hint, &mut rng);
            black_box(verify_zk_10round(&pp, &c, MSG, &proofs));
            proof_size =
                (proofs[0].t1.len() * 8 + proofs[0].sr.len() * 8 + proofs[0].se.len() * 8) * 10;
        }
        let elapsed = start.elapsed();
        let time_ms = elapsed.as_secs_f64() * 1000.0 / trials as f64;

        wtr.write_record([
            n.to_string(),
            "10".to_string(),
            time_ms.to_string(),
            proof_size.to_string(),
        ])
        .unwrap();
    }
    wtr.flush().unwrap();
}
