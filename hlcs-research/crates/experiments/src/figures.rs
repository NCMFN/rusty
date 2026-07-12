use crate::{
    exp_a::ExpAResult, exp_b::ExpBResult, exp_c::ExpCResult, exp_d::ExpDResult, exp_f::ExpFResult,
    exp_g::ExpGResult, exp_h::ExpHResult, exp_i::ExpIResult, exp_j::ExpJResult, exp_k::ExpKResult,
};
use anyhow::Result;
use forex_sim::BurstResult;
use plotters::prelude::*;

pub fn generate_all(
    a: &ExpAResult,
    b: &ExpBResult,
    c: &ExpCResult,
    _d: &ExpDResult,
    e: &BurstResult,
    f: &ExpFResult,
    g: &ExpGResult,
    h: &ExpHResult,
    i: &ExpIResult,
    j: &ExpJResult,
    k: &ExpKResult,
) -> Result<()> {
    f01_commitment_latency_comparison(a)?;
    f02_throughput_comparison(b)?;
    f03_communication_overhead(k)?;
    f04_latency_heatmap_dimension_vs_load(c)?;
    f05_eurusd_tick_series(g)?;
    f06_forex_latency_distribution()?;
    f07_latency_per_order_over_time(f)?;
    f08_burst_simulation(e)?;
    f09_decryption_failure_vs_sigma_multiple(h)?;
    f10_binding_bound_vs_qh(j)?;
    f11_las_security_margin(j)?;
    f12_zk_proof_timing(i)?;
    f13_zk_completeness_soundness(i)?;
    f14_latency_vs_dimension_all_schemes(c)?;
    f15_sequence_diagram()?;
    f16_architecture_diagram()?;
    f17_radar_comparison()?;
    f18_ablation_two_tier_vs_full_verify(k)?;
    f19_placeholder()?;
    f20_placeholder()?;
    Ok(())
}

fn draw_bar_chart_str(filename: &str, title: &str, data: &[(&str, f64)], y_log: f64) -> Result<()> {
    let root = BitMapBackend::new(filename, (1500, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_y = data
        .iter()
        .map(|&(_, v)| v)
        .fold(f64::NEG_INFINITY, f64::max)
        * 1.2;
    let num_bars = data.len();
    let names: Vec<String> = data.iter().map(|(n, _)| n.to_string()).collect();

    if y_log > 0.0 {
        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 30))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(0..num_bars as i32, (y_log..max_y).log_scale())?;

        chart
            .configure_mesh()
            .x_labels(num_bars)
            .x_label_formatter(&|v| {
                if *v >= 0 && (*v as usize) < names.len() {
                    names[*v as usize].clone()
                } else {
                    "".to_string()
                }
            })
            .draw()?;

        for (i, &(_, val)) in data.iter().enumerate() {
            let x0 = i as i32;
            let x1 = x0 + 1;
            let y = val.max(y_log);
            chart.draw_series(std::iter::once(Rectangle::new(
                [(x0, y_log), (x1, y)],
                BLUE.filled(),
            )))?;
        }
    } else {
        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 30))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(0..num_bars as i32, 0f64..max_y)?;

        chart
            .configure_mesh()
            .x_labels(num_bars)
            .x_label_formatter(&|v| {
                if *v >= 0 && (*v as usize) < names.len() {
                    names[*v as usize].clone()
                } else {
                    "".to_string()
                }
            })
            .draw()?;

        for (i, &(_, val)) in data.iter().enumerate() {
            let x0 = i as i32;
            let x1 = x0 + 1;
            let y = val;
            chart.draw_series(std::iter::once(Rectangle::new(
                [(x0, 0.0), (x1, y)],
                BLUE.filled(),
            )))?;
        }
    }

    Ok(())
}

fn f01_commitment_latency_comparison(res: &ExpAResult) -> Result<()> {
    let hash_ms =
        res.hash_latencies_us.iter().sum::<f64>() / res.hash_latencies_us.len() as f64 / 1000.0;
    let lattice_ms = res.lattice_latencies_us.iter().sum::<f64>()
        / res.lattice_latencies_us.len() as f64
        / 1000.0;
    let hybrid_ms =
        res.hybrid_latencies_us.iter().sum::<f64>() / res.hybrid_latencies_us.len() as f64 / 1000.0;

    draw_bar_chart_str(
        "figures/F01_commitment_latency_comparison.png",
        "Figure 1: Commitment Latency Comparison",
        &[
            ("Hash", hash_ms),
            ("Lattice", lattice_ms),
            ("Hybrid", hybrid_ms),
        ],
        0.01,
    )
}

fn f02_throughput_comparison(res: &ExpBResult) -> Result<()> {
    draw_bar_chart_str(
        "figures/F02_throughput_comparison.png",
        "Figure 2: Throughput Comparison",
        &[
            ("Hash (S)", res.hash_tps_single),
            ("Hash (M)", res.hash_tps_multi),
            ("Latt (S)", res.lattice_tps_single),
            ("Latt (M)", res.lattice_tps_multi),
            ("Hyb (S)", res.hybrid_tps_single),
            ("Hyb (M)", res.hybrid_tps_multi),
        ],
        100.0,
    )
}

fn f03_communication_overhead(res: &ExpKResult) -> Result<()> {
    draw_bar_chart_str(
        "figures/F03_communication_overhead.png",
        "Figure 3: Communication Overhead (Bytes)",
        &[
            ("Hash", res.comm_data[0].bytes as f64),
            ("Lattice", res.comm_data[1].bytes as f64),
            ("Hybrid", res.comm_data[2].bytes as f64),
        ],
        0.0,
    )
}

fn f04_latency_heatmap_dimension_vs_load(_res: &ExpCResult) -> Result<()> {
    let root = BitMapBackend::new(
        "figures/F04_latency_heatmap_dimension_vs_load.png",
        (1000, 1000),
    )
    .into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Figure 4: Hybrid Latency Heatmap (Dimension vs Load)",
            ("sans-serif", 30),
        )
        .margin(20)
        .build_cartesian_2d(0..5, 0..5)?;
    chart.configure_mesh().draw()?;
    Ok(())
}

fn f05_eurusd_tick_series(res: &ExpGResult) -> Result<()> {
    let root =
        BitMapBackend::new("figures/F05_eurusd_tick_series.png", (1500, 800)).into_drawing_area();
    root.fill(&WHITE)?;
    let min_p = res
        .ticks
        .iter()
        .map(|t| t.price)
        .fold(f64::INFINITY, f64::min);
    let max_p = res
        .ticks
        .iter()
        .map(|t| t.price)
        .fold(f64::NEG_INFINITY, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Figure 5: Simulated EUR/USD Tick Series",
            ("sans-serif", 30),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0f64..res.ticks.last().unwrap().time_ms, min_p..max_p)?;

    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
        res.ticks.iter().map(|t| (t.time_ms, t.price)),
        &BLUE,
    ))?;
    Ok(())
}

fn f06_forex_latency_distribution() -> Result<()> {
    let root = BitMapBackend::new("figures/F06_forex_latency_distribution.png", (1500, 800))
        .into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Figure 6: Forex Latency Distribution (Hybrid)",
            ("sans-serif", 30),
        )
        .margin(20)
        .build_cartesian_2d(0..1, 0..1)?;
    chart.configure_mesh().draw()?;
    Ok(())
}

fn f07_latency_per_order_over_time(res: &ExpFResult) -> Result<()> {
    let root = BitMapBackend::new("figures/F07_latency_per_order_over_time.png", (1500, 800))
        .into_drawing_area();
    root.fill(&WHITE)?;
    let max_y = res
        .latencies
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max)
        .max(res.threshold_ms)
        * 1.2;
    let mut chart = ChartBuilder::on(&root)
        .caption("Figure 7: Latency Per Order Over Time", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0f64..res.latencies.len() as f64, 0f64..max_y)?;

    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
        res.latencies
            .iter()
            .enumerate()
            .map(|(i, &v)| (i as f64, v)),
        &BLUE,
    ))?;
    chart.draw_series(LineSeries::new(
        res.rolling_means
            .iter()
            .enumerate()
            .map(|(i, &v)| (i as f64, v)),
        &RED,
    ))?;
    Ok(())
}

fn f08_burst_simulation(res: &BurstResult) -> Result<()> {
    draw_bar_chart_str(
        "figures/F08_burst_simulation.png",
        "Figure 8: Burst Simulation (1ms window)",
        &[
            ("Target", res.target_orders as f64),
            ("Processed", res.orders_processed as f64),
            ("Dropped", res.orders_dropped as f64),
        ],
        0.0,
    )
}

fn f09_decryption_failure_vs_sigma_multiple(res: &ExpHResult) -> Result<()> {
    let root = BitMapBackend::new(
        "figures/F09_decryption_failure_vs_sigma_multiple.png",
        (1500, 800),
    )
    .into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Figure 9: Decryption Failure vs B/sigma",
            ("sans-serif", 30),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(1.0f64..6.0f64, (1e-10f64..1.0f64).log_scale())?;

    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
        res.data
            .iter()
            .map(|p| (p.b_over_sigma, p.per_commitment_bound.max(1e-10))),
        &BLUE,
    ))?;
    Ok(())
}

fn f10_binding_bound_vs_qh(res: &ExpJResult) -> Result<()> {
    let root =
        BitMapBackend::new("figures/F10_binding_bound_vs_qh.png", (1500, 800)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Figure 10: Binding Bound vs Hash Queries",
            ("sans-serif", 30),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(
            (1e6f64..1e21f64).log_scale(),
            (1e-50f64..1.0f64).log_scale(),
        )?;

    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
        res.binding_data
            .iter()
            .map(|p| (p.q_hash, p.collision_bound.max(1e-50))),
        &RED,
    ))?;
    Ok(())
}

fn f11_las_security_margin(res: &ExpJResult) -> Result<()> {
    let data: Vec<(String, f64)> = res
        .las_data
        .iter()
        .map(|p| (format!("n={}", p.n), p.margin.margin_orders_of_magnitude))
        .collect();
    let data_refs: Vec<(&str, f64)> = data.iter().map(|(s, v)| (s.as_str(), *v)).collect();
    draw_bar_chart_str(
        "figures/F11_las_security_margin.png",
        "Figure 11: LAS Security Margin",
        &data_refs,
        0.0,
    )
}

fn f12_zk_proof_timing(res: &ExpIResult) -> Result<()> {
    draw_bar_chart_str(
        "figures/F12_zk_proof_timing.png",
        "Figure 12: ZK Proof Timing (us)",
        &[
            ("Prove", res.prove_time_us.mean_ms * 1000.0),
            ("Verify", res.verify_time_us.mean_ms * 1000.0),
        ],
        0.0,
    )
}

fn f13_zk_completeness_soundness(res: &ExpIResult) -> Result<()> {
    draw_bar_chart_str(
        "figures/F13_zk_completeness_soundness.png",
        "Figure 13: ZK Completeness vs Soundness Violation Rate",
        &[
            ("Completeness", res.completeness_rate),
            ("Soundness V.", res.soundness_violation_rate),
        ],
        0.0,
    )
}

fn f14_latency_vs_dimension_all_schemes(res: &ExpCResult) -> Result<()> {
    let root = BitMapBackend::new(
        "figures/F14_latency_vs_dimension_all_schemes.png",
        (1500, 800),
    )
    .into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Figure 14: Latency vs Dimension", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(128f64..1024f64, 0f64..20.0f64)?;

    chart.configure_mesh().draw()?;
    let subset: Vec<_> = res.data.iter().filter(|p| p.order_load == 100).collect();
    chart.draw_series(LineSeries::new(
        subset.iter().map(|p| (p.n as f64, p.hash_mean_ms)),
        &RED,
    ))?;
    chart.draw_series(LineSeries::new(
        subset.iter().map(|p| (p.n as f64, p.lattice_mean_ms)),
        &BLUE,
    ))?;
    chart.draw_series(LineSeries::new(
        subset.iter().map(|p| (p.n as f64, p.hybrid_mean_ms)),
        &GREEN,
    ))?;
    Ok(())
}

fn f15_sequence_diagram() -> Result<()> {
    let root =
        BitMapBackend::new("figures/F15_sequence_diagram.png", (1500, 800)).into_drawing_area();
    root.fill(&WHITE)?;
    root.draw(&Text::new(
        "Sequence Diagram Placeholder",
        (100, 100),
        ("sans-serif", 40),
    ))?;
    Ok(())
}

fn f16_architecture_diagram() -> Result<()> {
    let root =
        BitMapBackend::new("figures/F16_architecture_diagram.png", (1500, 800)).into_drawing_area();
    root.fill(&WHITE)?;
    root.draw(&Text::new(
        "Architecture Diagram Placeholder",
        (100, 100),
        ("sans-serif", 40),
    ))?;
    Ok(())
}

fn f17_radar_comparison() -> Result<()> {
    let root =
        BitMapBackend::new("figures/F17_radar_comparison.png", (1000, 1000)).into_drawing_area();
    root.fill(&WHITE)?;
    root.draw(&Text::new(
        "Radar Chart Placeholder",
        (100, 100),
        ("sans-serif", 40),
    ))?;
    Ok(())
}

fn f18_ablation_two_tier_vs_full_verify(res: &ExpKResult) -> Result<()> {
    let sc_mean = res
        .ablation
        .short_circuit_verify_latencies_us
        .iter()
        .sum::<f64>()
        / res.ablation.short_circuit_verify_latencies_us.len() as f64
        / 1000.0;
    let full_mean = res
        .ablation
        .full_lattice_verify_latencies_us
        .iter()
        .sum::<f64>()
        / res.ablation.full_lattice_verify_latencies_us.len() as f64
        / 1000.0;

    draw_bar_chart_str(
        "figures/F18_ablation_two_tier_vs_full_verify.png",
        "Figure 18: Ablation: Verify Latency (ms)",
        &[("Short Circuit", sc_mean), ("Full Lattice", full_mean)],
        0.0,
    )
}

fn f19_placeholder() -> Result<()> {
    let root = BitMapBackend::new("figures/F19_placeholder.png", (1000, 1000)).into_drawing_area();
    root.fill(&WHITE)?;
    root.draw(&Text::new("Figure 19 Placeholder", (100, 100), ("sans-serif", 40)))?;
    Ok(())
}

fn f20_placeholder() -> Result<()> {
    let root = BitMapBackend::new("figures/F20_placeholder.png", (1000, 1000)).into_drawing_area();
    root.fill(&WHITE)?;
    root.draw(&Text::new("Figure 20 Placeholder", (100, 100), ("sans-serif", 40)))?;
    Ok(())
}
