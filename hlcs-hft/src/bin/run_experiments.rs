use csv::Writer;
use hlcs_hft::*;
use rand::{rngs::StdRng, SeedableRng};
use std::fs::create_dir_all;
use std::time::Instant;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn get_msg() -> Vec<u8> {
    b"FX_ORDER:SYM=EURUSD;SIDE=BUY;PX=1.1234;SZ=1M".to_vec()
}

fn measure_latency<F>(mut f: F, iters: usize) -> Vec<f64>
where
    F: FnMut(),
{
    let mut latencies = Vec::with_capacity(iters);
    for _ in 0..iters {
        let start = Instant::now();
        f();
        latencies.push(start.elapsed().as_nanos() as f64 / 1_000_000.0); // ms
    }
    latencies
}

fn stats(data: &[f64]) -> (f64, f64, f64) {
    if data.is_empty() {
        return (0.0, 0.0, 0.0);
    }
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let sum: f64 = data.iter().sum();
    let mean = sum / data.len() as f64;

    let p99_idx = (data.len() as f64 * 0.99) as usize;
    let p99 = sorted[p99_idx.min(data.len() - 1)];

    // 95% CI roughly = 1.96 * std_dev / sqrt(n)
    let variance = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;
    let std_dev = variance.sqrt();
    let ci95 = 1.96 * std_dev / (data.len() as f64).sqrt();

    (mean, p99, ci95)
}

fn run_base_sweep() {
    let mut wtr = Writer::from_path("bench_results/base_sweep.csv").unwrap();
    wtr.write_record(["scheme", "n", "mean_ms", "p99_ms", "ci95_ms", "size_bytes"])
        .unwrap();

    let ns = vec![128, 192, 256, 384, 512, 640, 768, 896, 1024];
    let msg = get_msg();
    let iters = 5000;

    let mut rng = StdRng::seed_from_u64(42);

    for &n in &ns {
        let pp = PublicParams::new(n, &mut rng);
        let r_hash = vec![42; n];

        // Hash only
        let latencies = measure_latency(
            || {
                let _ = commit_hash_only(&msg, &r_hash);
            },
            iters,
        );
        let (mean, p99, ci95) = stats(&latencies);
        wtr.write_record([
            "hash_only",
            &n.to_string(),
            &mean.to_string(),
            &p99.to_string(),
            &ci95.to_string(),
            "32",
        ])
        .unwrap();

        // Hybrid
        let latencies = measure_latency(
            || {
                let _ = commit(&pp, &msg, &mut rng, SIGMA);
            },
            iters,
        );
        let (mean, p99, ci95) = stats(&latencies);
        let size = 32 + n * 8; // approx c1 + c2
        wtr.write_record([
            "hybrid",
            &n.to_string(),
            &mean.to_string(),
            &p99.to_string(),
            &ci95.to_string(),
            &size.to_string(),
        ])
        .unwrap();

        // Lattice only
        let latencies = measure_latency(
            || {
                let _ = commit_lattice_only(&pp, &msg, &mut rng, SIGMA);
            },
            iters,
        );
        let (mean, p99, ci95) = stats(&latencies);
        let size = n * 8;
        wtr.write_record([
            "lattice_only",
            &n.to_string(),
            &mean.to_string(),
            &p99.to_string(),
            &ci95.to_string(),
            &size.to_string(),
        ])
        .unwrap();
    }
    wtr.flush().unwrap();
}

fn run_hardware_ablation() {
    let mut wtr = Writer::from_path("bench_results/hardware_ablation.csv").unwrap();
    wtr.write_record(["mode", "n", "mean_ms"]).unwrap();

    let ns = vec![256, 512, 768, 1024];
    let msg = get_msg();
    let iters = 1000;
    let mut rng = StdRng::seed_from_u64(42);

    for &n in &ns {
        // Cold cache (new PP each time)
        let latencies = measure_latency(
            || {
                let pp = PublicParams::new(n, &mut rng);
                let _ = commit(&pp, &msg, &mut rng, SIGMA);
            },
            iters,
        );
        let (mean, _, _) = stats(&latencies);
        wtr.write_record(["cold_cache", &n.to_string(), &mean.to_string()])
            .unwrap();

        // Warm cache
        let pp = PublicParams::new(n, &mut rng);
        let latencies = measure_latency(
            || {
                let _ = commit(&pp, &msg, &mut rng, SIGMA);
            },
            iters,
        );
        let (mean, _, _) = stats(&latencies);
        wtr.write_record(["warm_cache", &n.to_string(), &mean.to_string()])
            .unwrap();
    }
    wtr.flush().unwrap();
}

fn run_workload_variation() {
    let mut wtr = Writer::from_path("bench_results/workload_variation.csv").unwrap();
    wtr.write_record(["msg_size", "mean_ms"]).unwrap();

    let sizes = vec![32, 128, 256, 512, 1024];
    let n = 512;
    let iters = 2000;
    let mut rng = StdRng::seed_from_u64(42);
    let pp = PublicParams::new(n, &mut rng);

    for &size in &sizes {
        let msg = vec![42u8; size];
        let latencies = measure_latency(
            || {
                let _ = commit(&pp, &msg, &mut rng, SIGMA);
            },
            iters,
        );
        let (mean, _, _) = stats(&latencies);
        wtr.write_record([&size.to_string(), &mean.to_string()])
            .unwrap();
    }
    wtr.flush().unwrap();
}

fn run_overload_grid() {
    // We simulate overload by sleeping to meet rate, but in a simple tight loop we just track how many exceed 1ms.
    let mut wtr = Writer::from_path("bench_results/overload_grid.csv").unwrap();
    wtr.write_record(["rate_per_sec", "n", "breach_rate"])
        .unwrap();

    let ns = vec![128, 256, 512, 768, 1024];
    let rates = vec![100, 500, 1000, 2500, 5000, 7500, 10000];
    let msg = get_msg();
    let mut rng = StdRng::seed_from_u64(42);

    for &rate in &rates {
        let _interval_ns = 1_000_000_000 / rate as u128;
        for &n in &ns {
            let pp = PublicParams::new(n, &mut rng);
            let mut breaches = 0;
            let iters = 1000;
            for _ in 0..iters {
                let start = Instant::now();
                let _ = commit(&pp, &msg, &mut rng, SIGMA);
                let elapsed = start.elapsed().as_nanos();
                if elapsed > 1_000_000 {
                    // 1ms
                    breaches += 1;
                }
            }
            let breach_rate = breaches as f64 / iters as f64;
            wtr.write_record([&rate.to_string(), &n.to_string(), &breach_rate.to_string()])
                .unwrap();
        }
    }
    wtr.flush().unwrap();
}

fn run_zk_round_sweep() {
    let mut wtr = Writer::from_path("bench_results/zk_sweep.csv").unwrap();
    wtr.write_record(["rounds", "n", "prove_ms", "verify_ms", "proof_size"])
        .unwrap();

    let ns = vec![256, 512, 1024];
    let rounds_list = vec![1, 2, 3, 5, 7, 10, 15, 20];
    let msg = get_msg();
    let iters = 500;
    let mut rng = StdRng::seed_from_u64(42);

    for &n in &ns {
        let pp = PublicParams::new(n, &mut rng);
        let (c, hint) = commit(&pp, &msg, &mut rng, SIGMA);

        for &rounds in &rounds_list {
            let mut latencies_prove = Vec::with_capacity(iters);
            let mut latencies_verify = Vec::with_capacity(iters);

            let mut size = 0;
            for _ in 0..iters {
                let start = Instant::now();
                let proof = prove_zk(&pp, &c, &hint, rounds, &mut rng, SIGMA);
                latencies_prove.push(start.elapsed().as_nanos() as f64 / 1_000_000.0);

                size = (proof.t1_list.len() + proof.sr_list.len() + proof.se_list.len()) * n * 8;

                let start_v = Instant::now();
                verify_zk(&pp, &c, &proof);
                latencies_verify.push(start_v.elapsed().as_nanos() as f64 / 1_000_000.0);
            }

            let (mean_p, _, _) = stats(&latencies_prove);
            let (mean_v, _, _) = stats(&latencies_verify);
            wtr.write_record([
                &rounds.to_string(),
                &n.to_string(),
                &mean_p.to_string(),
                &mean_v.to_string(),
                &size.to_string(),
            ])
            .unwrap();
        }
    }
    wtr.flush().unwrap();
}

// Correctness properties are typically written as #[test] in lib.rs or tests/.
// Here we can run a simple manual randomized check if needed, but proptest will be added to a test suite.

fn main() {
    create_dir_all("bench_results").unwrap();
    let _profiler = dhat::Profiler::new_heap();

    println!("Running base sweep...");
    run_base_sweep();

    println!("Running parameter sensitivity...");
    run_parameter_sensitivity();

    println!("Running hardware ablation...");
    run_hardware_ablation();

    println!("Running workload variation...");
    run_workload_variation();

    println!("Running overload grid...");
    run_overload_grid();

    println!("Running ZK round sweep...");
    run_zk_round_sweep();

    println!("Done!");
}

fn run_parameter_sensitivity() {
    let mut wtr = Writer::from_path("bench_results/param_sensitivity.csv").unwrap();
    wtr.write_record(["sigma", "mean_ms", "p99_ms", "ci95_ms"])
        .unwrap();
    let n = 512;
    let sigmas = vec![1.0, 2.0, 3.2, 4.5, 6.0];
    let msg = get_msg();
    let iters = 2000;
    let mut rng = StdRng::seed_from_u64(42);
    let pp = PublicParams::new(n, &mut rng);

    for &sigma in &sigmas {
        let latencies = measure_latency(
            || {
                let _ = commit(&pp, &msg, &mut rng, sigma);
            },
            iters,
        );
        let (mean, p99, ci95) = stats(&latencies);
        wtr.write_record([
            &sigma.to_string(),
            &mean.to_string(),
            &p99.to_string(),
            &ci95.to_string(),
        ])
        .unwrap();
    }
    wtr.flush().unwrap();
}
