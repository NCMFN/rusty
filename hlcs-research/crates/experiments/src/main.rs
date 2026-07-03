pub mod exp_a;
pub mod exp_b;
pub mod exp_c;
pub mod exp_d;
pub mod exp_e;
pub mod exp_f;
pub mod exp_g;
pub mod exp_h;
pub mod exp_i;
pub mod exp_j;
pub mod exp_k;
pub mod figures;
pub mod report;
pub mod tables;

use anyhow::Result;
use std::fs;

fn load_config() -> tables::Config {
    tables::Config {}
}

fn main() -> Result<()> {
    println!("Starting experiments...");

    fs::create_dir_all("figures")?;
    fs::create_dir_all("tables")?;
    fs::create_dir_all("data")?;
    fs::create_dir_all("report")?;

    let cfg = load_config();
    let seed = 42;
    let q = 12289;
    let sigma = 3.2;
    let n = 512;

    println!("Running Exp A...");
    let res_a = exp_a::run(seed, 100, n, q, sigma);

    println!("Running Exp B...");
    let res_b = exp_b::run(seed, 100, n, q, sigma);

    println!("Running Exp C...");
    let res_c = exp_c::run(seed, &[128, 256, 512], &[10, 50, 100], q, sigma, 10);

    println!("Running Exp D...");
    let res_d = exp_d::run(seed, 10, 10, n, q, sigma);

    println!("Running Exp E...");
    let res_e = exp_e::run(seed, 100_000, 1.0, n, q, sigma);

    println!("Running Exp F...");
    let res_f = exp_f::run(&res_d.all_latencies_ms, 10, 1.0);

    println!("Running Exp G...");
    let res_g = exp_g::run(seed, 1000);

    println!("Running Exp H...");
    let res_h = exp_h::run(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], n, sigma);

    println!("Running Exp I...");
    let res_i = exp_i::run(seed, 100, n, q, sigma);

    println!("Running Exp J...");
    let res_j = exp_j::run(&[1e6, 1e9, 1e12, 1e15], &[256, 512, 768, 1024], 1.0);

    println!("Running Exp K...");
    let res_k = exp_k::run(seed, 100, n, q, sigma);

    println!("Generating figures...");
    figures::generate_all(
        &res_a, &res_b, &res_c, &res_d, &res_e, &res_f, &res_g, &res_h, &res_i, &res_j, &res_k,
    )?;

    println!("Generating tables...");
    tables::write_all(
        &res_a, &res_b, &res_c, &res_d, &res_e, &res_f, &res_g, &res_h, &res_i, &res_j, &res_k,
        &cfg,
    )?;

    println!("Generating report...");
    report::write("report/results_report.md", &res_a, &res_b, &res_e)?;

    println!("✅ All figures and tables generated.");
    Ok(())
}
