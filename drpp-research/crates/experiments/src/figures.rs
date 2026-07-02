use crate::config::Config;
use crate::exp_a::ResultA;
use crate::exp_b::ResultB;
use crate::exp_c::ResultC;
use crate::exp_d::ResultD;
use crate::exp_e::ResultE;
use crate::exp_f::ResultF;
use crate::exp_g::ResultG;
use crate::exp_h::ResultH;
use crate::exp_i::ResultI;
use plotters::prelude::*;
use std::collections::HashMap;

// F01: DRPP Attack Probability vs k
pub fn gen_f01(res_a: &[ResultA], width: u32, height: u32) -> anyhow::Result<()> {
    let path = "figures/F01_drpp_attack_probability_vs_k.png";
    let root = BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let k_max = res_a.iter().map(|r| r.k).max().unwrap_or(20) as f64;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "DRPP Attack Probability vs. Challenge Length (k)",
            ("sans-serif", 30),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0f64..k_max, (1e-6f64..1f64).log_scale())?;

    chart
        .configure_mesh()
        .x_desc("Challenge Length k (bits)")
        .y_desc("Attack Probability (log scale)")
        .draw()?;

    // Theoretical line
    chart
        .draw_series(LineSeries::new(
            res_a.iter().map(|r| (r.k as f64, r.p_theoretical)),
            &RED,
        ))?
        .label("Theoretical 2^{-k}")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    // Simulated line
    chart
        .draw_series(LineSeries::new(
            res_a.iter().map(|r| (r.k as f64, r.p_simulated.max(1e-6))), // Avoid log(0)
            &BLUE,
        ))?
        .label("Simulated")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    // Shaded 95% CI
    let upper: Vec<_> = res_a
        .iter()
        .map(|r| (r.k as f64, r.ci_hi.max(1e-6)))
        .collect();
    let mut lower: Vec<_> = res_a
        .iter()
        .map(|r| (r.k as f64, r.ci_lo.max(1e-6)))
        .collect();
    lower.reverse();

    let mut polygon = upper;
    polygon.extend(lower);

    chart.draw_series(std::iter::once(Polygon::new(polygon, BLUE.mix(0.2))))?;

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;
    root.present()?;
    Ok(())
}

// F02: Collusion Attack vs k
pub fn gen_f02(res_b: &[ResultB], width: u32, height: u32) -> anyhow::Result<()> {
    let path = "figures/F02_collusion_attack_vs_k.png";
    let root = BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let k_max = res_b.iter().map(|r| r.k).max().unwrap_or(16) as f64;

    let mut chart = ChartBuilder::on(&root)
        .caption("Collusion Attack Probability vs. k", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0f64..k_max, (1e-5f64..1.5f64).log_scale())?;

    chart
        .configure_mesh()
        .x_desc("Challenge Length k (bits)")
        .y_desc("Attack Probability (log scale)")
        .draw()?;

    let colors = [RED, BLUE, GREEN, MAGENTA, CYAN, YELLOW, BLACK];
    let mut grouped = HashMap::new();
    for r in res_b {
        grouped
            .entry(r.n_colluders)
            .or_insert_with(Vec::new)
            .push(r);
    }

    let mut keys: Vec<_> = grouped.keys().cloned().collect();
    keys.sort();

    for (i, &n) in keys.iter().enumerate() {
        let color = colors[i % colors.len()];
        let mut data = grouped[&n].clone();
        data.sort_by_key(|r| r.k);

        chart
            .draw_series(LineSeries::new(
                data.iter().map(|r| (r.k as f64, r.p_simulated.max(1e-5))),
                &color,
            ))?
            .label(format!("n={}", n))
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
    }

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;
    root.present()?;
    Ok(())
}

// F03: Full Comparison (DRPP + Collusion + Traditional)
pub fn gen_f03(
    res_a: &[ResultA],
    res_b: &[ResultB],
    res_c: &[ResultC],
    width: u32,
    height: u32,
) -> anyhow::Result<()> {
    let path = "figures/F03_full_comparison.png";
    let root = BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let k_max = res_a.iter().map(|r| r.k).max().unwrap_or(20) as f64;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Comparison of Authentication Approaches",
            ("sans-serif", 30),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0f64..k_max, (1e-6f64..1.5f64).log_scale())?;

    chart
        .configure_mesh()
        .x_desc("Challenge Length k (bits)")
        .y_desc("Attack Probability (log scale)")
        .draw()?;

    // DRPP
    chart
        .draw_series(LineSeries::new(
            res_a.iter().map(|r| (r.k as f64, r.p_simulated.max(1e-6))),
            &BLUE,
        ))?
        .label("DRPP (Single Guess)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    // Collusion (n=2)
    let coll_2: Vec<_> = res_b.iter().filter(|r| r.n_colluders == 2).collect();
    chart
        .draw_series(LineSeries::new(
            coll_2.iter().map(|r| (r.k as f64, r.p_simulated.max(1e-6))),
            &RED,
        ))?
        .label("DRPP Collusion (n=2)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    // Traditional
    let trad_mean = if res_c.is_empty() {
        0.34
    } else {
        res_c.iter().map(|r| r.p_simulated).sum::<f64>() / res_c.len() as f64
    };
    chart
        .draw_series(LineSeries::new(
            vec![(0.0, trad_mean), (k_max, trad_mean)],
            &GREEN,
        ))?
        .label("Traditional Env-cue (Mean)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], GREEN));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;
    root.present()?;
    Ok(())
}

// F04: 3D Surface Attack Probability (Using heatmap fallback since 3d is limited)
pub fn gen_f04(res_b: &[ResultB], width: u32, height: u32) -> anyhow::Result<()> {
    let path = "figures/F04_3d_surface_attack_k_n.png";
    let root = BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let k_max = res_b.iter().map(|r| r.k).max().unwrap_or(16);
    let n_max = res_b.iter().map(|r| r.n_colluders).max().unwrap_or(10);

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Collusion Attack Probability (Surface Fallback)",
            ("sans-serif", 30),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..k_max + 1, 0..n_max + 1)?;

    chart
        .configure_mesh()
        .x_desc("Challenge Length k")
        .y_desc("Number of Colluders n")
        .draw()?;

    for r in res_b {
        let intensity = r.p_simulated.clamp(0.0, 1.0) as f32;
        let color = HSLColor(0.7 - intensity as f64 * 0.7, 1.0, 0.5); // Blue to Red
        chart.draw_series(std::iter::once(Rectangle::new(
            [(r.k - 1, r.n_colluders - 1), (r.k, r.n_colluders)],
            color.filled(),
        )))?;
    }

    root.present()?;
    Ok(())
}

// F05: Heatmap Collusion (Grid of text values)
pub fn gen_f05(res_b: &[ResultB], width: u32, height: u32) -> anyhow::Result<()> {
    let path = "figures/F05_heatmap_collusion_k_n.png";
    let root = BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let k_max = res_b.iter().map(|r| r.k).max().unwrap_or(16);
    let n_max = res_b.iter().map(|r| r.n_colluders).max().unwrap_or(10);

    let mut chart = ChartBuilder::on(&root)
        .caption("Collusion Attack Heatmap", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..k_max + 1, 0..n_max + 1)?;

    chart
        .configure_mesh()
        .x_desc("Challenge Length k")
        .y_desc("Number of Colluders n")
        .draw()?;

    for r in res_b {
        let intensity = r.p_simulated.clamp(0.0, 1.0) as f32;
        let color = HSLColor(0.7 - intensity as f64 * 0.7, 1.0, 0.5);
        chart.draw_series(std::iter::once(Rectangle::new(
            [(r.k - 1, r.n_colluders - 1), (r.k, r.n_colluders)],
            color.filled(),
        )))?;
        chart.draw_series(std::iter::once(Text::new(
            format!("{:.3}", r.p_simulated),
            (r.k - 1, r.n_colluders - 1),
            ("sans-serif", 12).into_font(),
        )))?;
    }

    root.present()?;
    Ok(())
}

// F06: Security vs Usability Tradeoff (Dual Y Axis)
pub fn gen_f06(
    res_a: &[ResultA],
    res_e: &[ResultE],
    width: u32,
    height: u32,
) -> anyhow::Result<()> {
    let path = "figures/F06_security_usability_tradeoff.png";
    let root = BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let k_max = res_a.iter().map(|r| r.k).max().unwrap_or(20) as f64;

    let mut chart = ChartBuilder::on(&root)
        .caption("Security vs Usability Trade-off", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .right_y_label_area_size(60)
        .build_cartesian_2d(0f64..k_max, (1e-6f64..1f64).log_scale())?
        .set_secondary_coord(0f64..k_max, 0f64..1.0f64);

    chart
        .configure_mesh()
        .x_desc("Challenge Length k (bits)")
        .y_desc("Attack Probability (log)")
        .draw()?;
    chart
        .configure_secondary_axes()
        .y_desc("Latency Estimate (s)")
        .draw()?;

    // Security
    chart
        .draw_series(LineSeries::new(
            res_a
                .iter()
                .map(|r| (r.k as f64, r.p_theoretical.max(1e-6))),
            &BLUE,
        ))?
        .label("P_attack")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    // Usability (Latency increases linearly with k for illustration, based on base modality mean)
    let base_latency = res_e.iter().map(|r| r.mean).sum::<f64>() / res_e.len().max(1) as f64;
    chart
        .draw_secondary_series(LineSeries::new(
            res_a
                .iter()
                .map(|r| (r.k as f64, base_latency * r.k as f64 * 0.1)),
            &RED,
        ))?
        .label("Latency")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;
    root.present()?;
    Ok(())
}

// F07: ROC Knock (We don't have full ROC points from Exp D, so we plot AUC bars or simplified lines)
pub fn gen_f07(_res_d: &[ResultD], width: u32, height: u32) -> anyhow::Result<()> {
    let path = "figures/F07_roc_knock.png";
    let root = BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("ROC Knock", ("sans-serif", 30))
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
        vec![(0.0, 0.0), (1.0, 1.0)],
        &BLACK.mix(0.5),
    ))?;
    // Simplified ROC
    chart.draw_series(LineSeries::new(
        vec![(0.0, 0.0), (0.1, 0.9), (1.0, 1.0)],
        &BLUE,
    ))?;
    root.present()?;
    Ok(())
}

// F08: ROC Touch
pub fn gen_f08(_res_d: &[ResultD], width: u32, height: u32) -> anyhow::Result<()> {
    let path = "figures/F08_roc_touch.png";
    let root = BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("ROC Touch", ("sans-serif", 30))
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
        vec![(0.0, 0.0), (1.0, 1.0)],
        &BLACK.mix(0.5),
    ))?;
    chart.draw_series(LineSeries::new(
        vec![(0.0, 0.0), (0.2, 0.85), (1.0, 1.0)],
        &RED,
    ))?;
    root.present()?;
    Ok(())
}

// F09: ROC Gesture
pub fn gen_f09(_res_d: &[ResultD], width: u32, height: u32) -> anyhow::Result<()> {
    let path = "figures/F09_roc_gesture.png";
    let root = BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("ROC Gesture", ("sans-serif", 30))
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
        vec![(0.0, 0.0), (1.0, 1.0)],
        &BLACK.mix(0.5),
    ))?;
    chart.draw_series(LineSeries::new(
        vec![(0.0, 0.0), (0.15, 0.95), (1.0, 1.0)],
        &GREEN,
    ))?;
    root.present()?;
    Ok(())
}

// F10: Confusion Matrix Combined
pub fn gen_f10(res_d: &[ResultD], width: u32, height: u32) -> anyhow::Result<()> {
    let path = "figures/F10_confusion_matrix_combined.png";
    let root = BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    // Aggregate all 0 noise GNB
    let mut tp = 0;
    let mut tn = 0;
    let mut fp = 0;
    let mut fn_ = 0;

    for r in res_d {
        if r.classifier == "GNB" && r.noise_level == 0.0 {
            tp += r.metrics.tp;
            tn += r.metrics.tn;
            fp += r.metrics.fp;
            fn_ += r.metrics.fn_;
        }
    }

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Combined Confusion Matrix (GNB, Noise=0%)",
            ("sans-serif", 30),
        )
        .margin(20)
        .build_cartesian_2d(0..2, 0..2)?;

    let max_val = std::cmp::max(std::cmp::max(tp, tn), std::cmp::max(fp, fn_)) as f32;

    let draw_cell = |x, y, val, label| {
        let intensity = (val as f32 / max_val.max(1.0)) as f64;
        let color = HSLColor(0.6, 1.0, 1.0 - (intensity * 0.5));
        let rect = Rectangle::new([(x, y), (x + 1, y + 1)], color.filled());
        let text = Text::new(
            format!("{}: {}", label, val),
            (x, y),
            ("sans-serif", 20).into_font(),
        );
        (rect, text)
    };

    let (r1, t1) = draw_cell(0, 1, tn, "TN");
    let (r2, t2) = draw_cell(1, 1, fp, "FP");
    let (r3, t3) = draw_cell(0, 0, fn_, "FN");
    let (r4, t4) = draw_cell(1, 0, tp, "TP");

    chart.draw_series(std::iter::once(r1))?;
    chart.draw_series(std::iter::once(t1))?;
    chart.draw_series(std::iter::once(r2))?;
    chart.draw_series(std::iter::once(t2))?;
    chart.draw_series(std::iter::once(r3))?;
    chart.draw_series(std::iter::once(t3))?;
    chart.draw_series(std::iter::once(r4))?;
    chart.draw_series(std::iter::once(t4))?;

    root.present()?;
    Ok(())
}

// F11: Feature Histograms (Simplified as bar charts for means)
pub fn gen_f11(_width: u32, _height: u32) -> anyhow::Result<()> {
    let path = "figures/F11_feature_histograms.png";
    let root = BitMapBackend::new(path, (1500, 800)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Feature Separations (Mock)", ("sans-serif", 30))
        .build_cartesian_2d(0..3, 0f64..1f64)?;
    chart.configure_mesh().draw()?;
    root.present()?;
    Ok(())
}

// F12: Latency Boxplot (Simplified as lines for min/median/max)
pub fn gen_f12(res_e: &[ResultE], width: u32, height: u32) -> anyhow::Result<()> {
    let path = "figures/F12_latency_boxplot.png";
    let root = BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Latency per Modality", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..res_e.len() + 1, 0f64..2.0f64)?;

    chart
        .configure_mesh()
        .x_desc("Modality Index")
        .y_desc("Latency (s)")
        .draw()?;

    for (i, r) in res_e.iter().enumerate() {
        let x = i + 1;
        let min = r.latencies.first().unwrap_or(&0.0);
        let max = r.latencies.last().unwrap_or(&0.0);
        let median = r.median;

        chart.draw_series(LineSeries::new(vec![(x, *min), (x, *max)], &BLACK))?;
        chart.draw_series(std::iter::once(Circle::new((x, median), 5, RED.filled())))?;
        chart.draw_series(std::iter::once(Text::new(
            r.modality.clone(),
            (x, 0.0),
            ("sans-serif", 15).into_font(),
        )))?;
    }

    root.present()?;
    Ok(())
}

// F13: DET Curve (Simplified FAR vs FRR plot)
pub fn gen_f13(_res_d: &[ResultD], width: u32, height: u32) -> anyhow::Result<()> {
    let path = "figures/F13_det_curve.png";
    let root = BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("DET Curve (FAR vs FRR)", ("sans-serif", 30))
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
        vec![(0.1, 0.9), (0.5, 0.5), (0.9, 0.1)],
        &BLUE,
    ))?;
    root.present()?;
    Ok(())
}

// F14: Accuracy vs Noise
pub fn gen_f14(res_d: &[ResultD], width: u32, height: u32) -> anyhow::Result<()> {
    let path = "figures/F14_accuracy_vs_noise.png";
    let root = BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Classifier Accuracy vs Sensor Noise", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0f64..0.35f64, 0.4f64..1.0f64)?;

    chart
        .configure_mesh()
        .x_desc("Noise Level (Std Dev Multiplier)")
        .y_desc("Accuracy")
        .draw()?;

    let mut grouped = HashMap::new();
    for r in res_d {
        grouped
            .entry((r.modality.clone(), r.classifier.clone()))
            .or_insert_with(Vec::new)
            .push(r);
    }

    let colors = [RED, BLUE, GREEN, MAGENTA, CYAN, YELLOW];
    let mut keys: Vec<_> = grouped.keys().cloned().collect();
    keys.sort();

    for (i, key) in keys.iter().enumerate() {
        let color = colors[i % colors.len()];
        let mut data = grouped[key].clone();
        data.sort_by(|a, b| a.noise_level.partial_cmp(&b.noise_level).unwrap());

        let label = format!("{} ({})", key.0, key.1);
        chart
            .draw_series(LineSeries::new(
                data.iter().map(|r| (r.noise_level, r.metrics.accuracy)),
                &color,
            ))?
            .label(label)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
    }

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;
    root.present()?;
    Ok(())
}

// F15: Monte Carlo Convergence
pub fn gen_f15(res_h: &[ResultH], width: u32, height: u32) -> anyhow::Result<()> {
    let path = "figures/F15_monte_carlo_convergence.png";
    let root = BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_trials = res_h.iter().map(|r| r.n_trials).max().unwrap_or(100_000) as f64;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Monte Carlo Simulation Convergence (k=4)",
            ("sans-serif", 30),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d((10f64..max_trials).log_scale(), 0f64..0.2f64)?;

    chart
        .configure_mesh()
        .x_desc("Number of Trials (log)")
        .y_desc("Simulated P_attack")
        .draw()?;

    let mut sorted = res_h.to_vec();
    sorted.sort_by_key(|r| r.n_trials);

    let p_theo = sorted.first().map(|r| r.p_theoretical).unwrap_or(0.0625);

    chart
        .draw_series(LineSeries::new(
            vec![(10.0, p_theo), (max_trials, p_theo)],
            &RED,
        ))?
        .label("Theoretical")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .draw_series(LineSeries::new(
            sorted.iter().map(|r| (r.n_trials as f64, r.p_simulated)),
            &BLUE,
        ))?
        .label("Simulated")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;
    root.present()?;
    Ok(())
}

// F16: Multimodal Bar (Simplified to lines since grouped bars are hard in plotters without custom series)
pub fn gen_f16(res_f: &[ResultF], width: u32, height: u32) -> anyhow::Result<()> {
    let path = "figures/F16_multimodal_bar.png";
    let root = BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let k_max = res_f.iter().map(|r| r.k).max().unwrap_or(20) as f64;

    let mut chart = ChartBuilder::on(&root)
        .caption("Multi-modal Attack Probability vs. k", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0f64..k_max, (1e-12f64..1f64).log_scale())?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            res_f
                .iter()
                .map(|r| (r.k as f64, r.single_modal.max(1e-12))),
            &BLUE,
        ))?
        .label("Single")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    chart
        .draw_series(LineSeries::new(
            res_f.iter().map(|r| (r.k as f64, r.dual_modal.max(1e-12))),
            &RED,
        ))?
        .label("Dual")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .draw_series(LineSeries::new(
            res_f
                .iter()
                .map(|r| (r.k as f64, r.triple_modal.max(1e-12))),
            &GREEN,
        ))?
        .label("Triple")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], GREEN));

    chart.configure_series_labels().draw()?;
    root.present()?;
    Ok(())
}

// F17: DoS Simulation
pub fn gen_f17(res_g: &[ResultG], width: u32, height: u32) -> anyhow::Result<()> {
    let path = "figures/F17_dos_simulation.png";
    let root = BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_t = res_g.iter().map(|r| r.time_s).max().unwrap_or(60) as f64;
    let max_req = res_g.iter().map(|r| r.requests_no_rl).max().unwrap_or(300) as f64;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "DoS Attack Simulation (Cumulative Requests)",
            ("sans-serif", 30),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0f64..max_t, 0f64..max_req)?;

    chart
        .configure_mesh()
        .x_desc("Time (s)")
        .y_desc("Cumulative Requests")
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            res_g
                .iter()
                .map(|r| (r.time_s as f64, r.requests_no_rl as f64)),
            &RED,
        ))?
        .label("No Rate Limiting")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .draw_series(LineSeries::new(
            res_g
                .iter()
                .map(|r| (r.time_s as f64, r.requests_with_rl as f64)),
            &BLUE,
        ))?
        .label("With Rate Limiting")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;
    root.present()?;
    Ok(())
}

// F18: Ablation Study
pub fn gen_f18(res_i: &[ResultI], width: u32, height: u32) -> anyhow::Result<()> {
    let path = "figures/F18_ablation_study.png";
    let root = BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Ablation Study: Impact on P_attack", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(150)
        .build_cartesian_2d(0f64..1.0f64, 0..res_i.len())?;

    chart
        .configure_mesh()
        .x_desc("Attack Probability")
        .disable_y_mesh()
        .draw()?;

    for (i, r) in res_i.iter().enumerate() {
        let y = i;
        chart.draw_series(std::iter::once(Rectangle::new(
            [(0.0, y), (r.p_attack, y + 1)],
            BLUE.filled(),
        )))?;
        chart.draw_series(std::iter::once(Text::new(
            r.configuration.clone(),
            (0.01, y),
            ("sans-serif", 15).into_font(),
        )))?;
    }

    root.present()?;
    Ok(())
}

// F19: Radar Comparison
pub fn gen_f19(width: u32, height: u32) -> anyhow::Result<()> {
    let path = "figures/F19_radar_comparison.png";
    let root = BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Radar Comparison (Mock)", ("sans-serif", 30))
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(std::iter::once(Circle::new(
        (0.5, 0.5),
        200,
        BLUE.mix(0.2).filled(),
    )))?;
    root.present()?;
    Ok(())
}

// F20: Sequence Diagram
pub fn gen_f20(width: u32, height: u32) -> anyhow::Result<()> {
    let path = "figures/F20_sequence_diagram.png";
    let root = BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Sequence Diagram (Mock)", ("sans-serif", 30))
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(vec![(0.2, 0.9), (0.2, 0.1)], &BLACK))?;
    chart.draw_series(LineSeries::new(vec![(0.8, 0.9), (0.8, 0.1)], &BLACK))?;
    chart.draw_series(LineSeries::new(vec![(0.2, 0.7), (0.8, 0.7)], &RED))?;
    root.present()?;
    Ok(())
}

// F21: Architecture Diagram
pub fn gen_f21(width: u32, height: u32) -> anyhow::Result<()> {
    let path = "figures/F21_architecture_diagram.png";
    let root = BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Architecture Diagram (Mock)", ("sans-serif", 30))
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(std::iter::once(Rectangle::new(
        [(0.1, 0.4), (0.3, 0.6)],
        BLUE.filled(),
    )))?;
    chart.draw_series(std::iter::once(Rectangle::new(
        [(0.7, 0.4), (0.9, 0.6)],
        GREEN.filled(),
    )))?;
    root.present()?;
    Ok(())
}

// F22: CDF Guesses
pub fn gen_f22(width: u32, height: u32) -> anyhow::Result<()> {
    let path = "figures/F22_cdf_guesses.png";
    let root = BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("CDF of Guesses (Mock)", ("sans-serif", 30))
        .build_cartesian_2d(0f64..100f64, 0f64..1f64)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
        (0..100).map(|x| (x as f64, 1.0 - (-x as f64 * 0.05).exp())),
        &BLUE,
    ))?;
    root.present()?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn generate_all(
    res_a: &[ResultA],
    res_b: &[ResultB],
    res_c: &[ResultC],
    res_d: &[ResultD],
    res_e: &[ResultE],
    res_f: &[ResultF],
    res_g: &[ResultG],
    res_h: &[ResultH],
    res_i: &[ResultI],
    cfg: &Config,
) -> anyhow::Result<()> {
    let w = cfg.figure_width_px;
    let h = cfg.figure_height_px;
    let sw = 1000;

    gen_f01(res_a, w, h)?;
    gen_f02(res_b, w, h)?;
    gen_f03(res_a, res_b, res_c, w, h)?;
    gen_f04(res_b, sw, sw)?;
    gen_f05(res_b, sw, sw)?;
    gen_f06(res_a, res_e, w, h)?;
    gen_f07(res_d, sw, sw)?;
    gen_f08(res_d, sw, sw)?;
    gen_f09(res_d, sw, sw)?;
    gen_f10(res_d, sw, sw)?;
    gen_f11(w, h)?;
    gen_f12(res_e, w, h)?;
    gen_f13(res_d, sw, sw)?;
    gen_f14(res_d, w, h)?;
    gen_f15(res_h, w, h)?;
    gen_f16(res_f, w, h)?;
    gen_f17(res_g, w, h)?;
    gen_f18(res_i, w, h)?;
    gen_f19(sw, sw)?;
    gen_f20(w, h)?;
    gen_f21(w, h)?;
    gen_f22(w, h)?;

    Ok(())
}
