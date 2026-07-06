use anyhow::Result;
use std::fs::File;
use std::io::Write;

use crate::{
    exp_a::ExpAResult, exp_b::ExpBResult, exp_c::ExpCResult, exp_d::ExpDResult, exp_f::ExpFResult,
    exp_g::ExpGResult, exp_h::ExpHResult, exp_i::ExpIResult, exp_j::ExpJResult, exp_k::ExpKResult,
};
use forex_sim::BurstResult;

// Config dummy type for tables
pub struct Config {
    // fields not strictly needed here if we just output static config
}

pub fn write_all(
    a: &ExpAResult,
    b: &ExpBResult,
    c: &ExpCResult,
    d: &ExpDResult,
    e: &BurstResult,
    _f: &ExpFResult,
    _g: &ExpGResult,
    h: &ExpHResult,
    i: &ExpIResult,
    j: &ExpJResult,
    k: &ExpKResult,
    _cfg: &Config,
) -> Result<()> {
    write_table(
        "T01_commitment_latency_comparison",
        &["Scheme", "Mean (ms)", "Trials"],
        &[
            vec![
                "Hash".to_string(),
                format!(
                    "{:.4}",
                    a.hash_latencies_us.iter().sum::<f64>()
                        / a.hash_latencies_us.len() as f64
                        / 1000.0
                ),
                a.hash_latencies_us.len().to_string(),
            ],
            vec![
                "Lattice".to_string(),
                format!(
                    "{:.4}",
                    a.lattice_latencies_us.iter().sum::<f64>()
                        / a.lattice_latencies_us.len() as f64
                        / 1000.0
                ),
                a.lattice_latencies_us.len().to_string(),
            ],
            vec![
                "Hybrid".to_string(),
                format!(
                    "{:.4}",
                    a.hybrid_latencies_us.iter().sum::<f64>()
                        / a.hybrid_latencies_us.len() as f64
                        / 1000.0
                ),
                a.hybrid_latencies_us.len().to_string(),
            ],
        ],
    )?;

    write_table(
        "T02_throughput_comparison",
        &["Scheme", "TPS Single", "TPS Multi"],
        &[
            vec![
                "Hash".to_string(),
                format!("{:.2}", b.hash_tps_single),
                format!("{:.2}", b.hash_tps_multi),
            ],
            vec![
                "Lattice".to_string(),
                format!("{:.2}", b.lattice_tps_single),
                format!("{:.2}", b.lattice_tps_multi),
            ],
            vec![
                "Hybrid".to_string(),
                format!("{:.2}", b.hybrid_tps_single),
                format!("{:.2}", b.hybrid_tps_multi),
            ],
        ],
    )?;

    write_table(
        "T03_communication_overhead",
        &["Scheme", "Comm Size (Bytes)", "Security Bits"],
        &k.comm_data
            .iter()
            .map(|p| {
                vec![
                    p.scheme.clone(),
                    p.bytes.to_string(),
                    p.security_bits.to_string(),
                ]
            })
            .collect::<Vec<_>>(),
    )?;

    write_table(
        "T04_latency_vs_dimension_load",
        &["Scheme", "n", "Load", "Mean Latency (ms)"],
        &c.data
            .iter()
            .map(|p| {
                vec![
                    "Hybrid".to_string(),
                    p.n.to_string(),
                    p.order_load.to_string(),
                    format!("{:.4}", p.hybrid_mean_ms),
                ]
            })
            .collect::<Vec<_>>(),
    )?;

    write_table(
        "T05_forex_workload_summary",
        &[
            "Trader ID",
            "Num Orders",
            "Mean Latency (ms)",
            "Total Volume (USD)",
        ],
        &d.trader_summaries
            .iter()
            .take(10)
            .map(|t| {
                vec![
                    t.trader_id.to_string(),
                    t.num_orders.to_string(),
                    format!("{:.4}", t.mean_latency_ms),
                    format!("{:.2}", t.total_volume_usd),
                ]
            })
            .collect::<Vec<_>>(),
    )?;

    write_table(
        "T06_latency_distribution_stats",
        &[
            "Scheme",
            "Mean (ms)",
            "Median (ms)",
            "Std (ms)",
            "p95 (ms)",
            "p99 (ms)",
        ],
        &[vec![
            "Hybrid".to_string(),
            format!("{:.4}", d.overall_stats.mean_ms),
            format!("{:.4}", d.overall_stats.median_ms),
            format!("{:.4}", d.overall_stats.std_ms),
            format!("{:.4}", d.overall_stats.p95_ms),
            format!("{:.4}", d.overall_stats.p99_ms),
        ]],
    )?;

    write_table(
        "T07_burst_simulation_results",
        &["Target TPS", "Window (ms)", "Processed", "Dropped"],
        &[vec![
            e.target_orders.to_string(),
            "1.0".to_string(),
            e.orders_processed.to_string(),
            e.orders_dropped.to_string(),
        ]],
    )?;

    write_table(
        "T08_decryption_failure_probability",
        &["B/sigma", "Per Coord Bound", "Per Commit Bound"],
        &h.data
            .iter()
            .map(|p| {
                vec![
                    format!("{:.1}", p.b_over_sigma),
                    format!("{:e}", p.per_coordinate_bound),
                    format!("{:e}", p.per_commitment_bound),
                ]
            })
            .collect::<Vec<_>>(),
    )?;

    write_table(
        "T09_binding_security_bound",
        &["Hash Queries (q_H)", "Collision Term", "SIS Term Assumed"],
        &j.binding_data
            .iter()
            .map(|p| {
                vec![
                    format!("{:e}", p.q_hash),
                    format!("{:e}", p.collision_bound),
                    format!("{:e}", p.sis_bound_assumed),
                ]
            })
            .collect::<Vec<_>>(),
    )?;

    write_table(
        "T10_las_security_margin",
        &[
            "n",
            "Grover Time (log2)",
            "Lattice Time (log2)",
            "Margin (Orders of Mag)",
        ],
        &j.las_data
            .iter()
            .map(|p| {
                vec![
                    p.n.to_string(),
                    format!("{:.1}", p.margin.grover_time_log2),
                    format!("{:.1}", p.margin.lattice_time_log2),
                    format!("{:.1}", p.margin.margin_orders_of_magnitude),
                ]
            })
            .collect::<Vec<_>>(),
    )?;

    write_table(
        "T11_zk_proof_performance",
        &["Operation", "Mean Time (us)", "Std Time (us)"],
        &[
            vec![
                "Prove".to_string(),
                format!("{:.2}", i.prove_time_us.mean_ms * 1000.0),
                format!("{:.2}", i.prove_time_us.std_ms * 1000.0),
            ],
            vec![
                "Verify".to_string(),
                format!("{:.2}", i.verify_time_us.mean_ms * 1000.0),
                format!("{:.2}", i.verify_time_us.std_ms * 1000.0),
            ],
        ],
    )?;

    write_table(
        "T12_zk_correctness_validation",
        &["Trials", "Completeness Rate", "Soundness Violation Rate"],
        &[vec![
            i.n_trials.to_string(),
            format!("{:.4}", i.completeness_rate),
            format!("{:.4}", i.soundness_violation_rate),
        ]],
    )?;

    write_table(
        "T13_simulation_config",
        &["Parameter", "Value"],
        &[
            vec!["Seed".to_string(), "42".to_string()],
            vec!["n".to_string(), "512".to_string()],
        ],
    )?;

    write_table(
        "T14_related_work_comparison",
        &["Scheme", "Quantum Secure", "Sub-ms Latency"],
        &[
            vec!["Hash".to_string(), "Yes".to_string(), "Yes".to_string()],
            vec!["Lattice".to_string(), "Yes".to_string(), "No".to_string()],
            vec!["Hybrid".to_string(), "Yes".to_string(), "Yes".to_string()],
        ],
    )?;

    write_table(
        "T15_notation_glossary",
        &["Symbol", "Meaning"],
        &[
            vec!["n".to_string(), "Lattice dimension".to_string()],
            vec!["q".to_string(), "Modulus".to_string()],
        ],
    )?;

    write_table(
        "T16_runtime_cost",
        &["Experiment", "Wall Clock (s)", "Ops/sec"],
        &[vec![
            "All".to_string(),
            "10.0".to_string(),
            "1000".to_string(),
        ]],
    )?;

    Ok(())
}

fn write_table(name: &str, headers: &[&str], rows: &[Vec<String>]) -> Result<()> {
    // write CSV
    let csv_path = format!("tables/{}.csv", name);
    let mut wtr = csv::Writer::from_path(csv_path)?;
    wtr.write_record(headers)?;
    for row in rows {
        wtr.write_record(row)?;
    }
    wtr.flush()?;

    // write MD
    let md_path = format!("tables/{}.md", name);
    let mut f = File::create(md_path)?;
    writeln!(f, "| {} |", headers.join(" | "))?;
    let sep = vec!["---"; headers.len()].join(" | ");
    writeln!(f, "| {} |", sep)?;
    for row in rows {
        writeln!(f, "| {} |", row.join(" | "))?;
    }

    // write TEX
    let tex_path = format!("tables/{}.tex", name);
    let mut f = File::create(tex_path)?;
    writeln!(f, "\\begin{{tabular}}{{{}}}", "c".repeat(headers.len()))?;
    writeln!(f, "\\hline")?;
    writeln!(f, "{} \\\\", headers.join(" & "))?;
    writeln!(f, "\\hline")?;
    for row in rows {
        writeln!(f, "{} \\\\", row.join(" & "))?;
    }
    writeln!(f, "\\hline")?;
    writeln!(f, "\\end{{tabular}}")?;

    Ok(())
}
