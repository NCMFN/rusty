use experiments::config::load_config;
use experiments::exp_a;
use experiments::exp_b;
use experiments::exp_c;
use experiments::exp_d;
use experiments::exp_e;
use experiments::exp_f;
use experiments::exp_g;
use experiments::exp_h;
use experiments::exp_i;
use experiments::figures;
use experiments::report;
use experiments::tables;
use std::fs;

fn main() -> anyhow::Result<()> {
    let cfg = load_config("experiments/config.toml")?;

    fs::create_dir_all("figures")?;
    fs::create_dir_all("tables")?;
    fs::create_dir_all("data")?;
    fs::create_dir_all("report")?;

    println!("Running Exp A...");
    let res_a = exp_a::run(&cfg)?;
    println!("Running Exp B...");
    let res_b = exp_b::run(&cfg)?;
    println!("Running Exp C...");
    let res_c = exp_c::run(&cfg)?;
    println!("Running Exp D...");
    let res_d = exp_d::run(&cfg)?;
    println!("Running Exp E...");
    let res_e = exp_e::run(&cfg)?;
    println!("Running Exp F...");
    let res_f = exp_f::run(&cfg)?;
    println!("Running Exp G...");
    let res_g = exp_g::run(&cfg)?;
    println!("Running Exp H...");
    let res_h = exp_h::run(&cfg)?;
    println!("Running Exp I...");
    let res_i = exp_i::run(&cfg)?;

    println!("Generating figures...");
    figures::generate_all(
        &res_a, &res_b, &res_c, &res_d, &res_e, &res_f, &res_g, &res_h, &res_i, &cfg,
    )?;

    println!("Generating tables...");
    tables::write_all(
        &res_a, &res_b, &res_c, &res_d, &res_e, &res_f, &res_g, &res_h, &res_i, &cfg,
    )?;

    println!("Writing report...");
    report::write("report/results_report.md", &res_a, &res_b)?;

    println!("✅  All figures and tables generated.");
    Ok(())
}
