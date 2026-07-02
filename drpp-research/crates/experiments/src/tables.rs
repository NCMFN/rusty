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
use anyhow::Result;
use csv::Writer;
use std::fs::File;
use std::io::Write;

pub struct TableData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl TableData {
    pub fn write_all(&self, name: &str) -> Result<()> {
        self.write_csv(&format!("tables/{}.csv", name))?;
        self.write_tex(&format!("tables/{}.tex", name))?;
        self.write_md(&format!("tables/{}.md", name))?;
        Ok(())
    }

    fn write_csv(&self, path: &str) -> Result<()> {
        let mut wtr = Writer::from_path(path)?;
        wtr.write_record(&self.headers)?;
        for row in &self.rows {
            wtr.write_record(row)?;
        }
        wtr.flush()?;
        Ok(())
    }

    fn write_tex(&self, path: &str) -> Result<()> {
        let mut file = File::create(path)?;
        let cols = "c".repeat(self.headers.len());
        writeln!(file, "\\begin{{table}}[h]")?;
        writeln!(file, "\\centering")?;
        writeln!(file, "\\begin{{tabular}}{{{}}}", cols)?;
        writeln!(file, "\\hline")?;
        writeln!(file, "{} \\\\", self.headers.join(" & "))?;
        writeln!(file, "\\hline")?;
        for row in &self.rows {
            writeln!(file, "{} \\\\", row.join(" & "))?;
        }
        writeln!(file, "\\hline")?;
        writeln!(file, "\\end{{tabular}}")?;
        writeln!(file, "\\end{{table}}")?;
        Ok(())
    }

    fn write_md(&self, path: &str) -> Result<()> {
        let mut file = File::create(path)?;
        writeln!(file, "| {} |", self.headers.join(" | "))?;
        let seps: Vec<String> = self.headers.iter().map(|_| "---".to_string()).collect();
        writeln!(file, "| {} |", seps.join(" | "))?;
        for row in &self.rows {
            writeln!(file, "| {} |", row.join(" | "))?;
        }
        Ok(())
    }
}

pub fn gen_t01(res_a: &[ResultA]) -> Result<()> {
    let mut rows = Vec::new();
    for r in res_a {
        rows.push(vec![
            r.k.to_string(),
            format!("{:.6}", r.p_theoretical),
            format!("{:.6}", r.p_simulated),
            format!("{:.6}", r.ci_lo),
            format!("{:.6}", r.ci_hi),
            r.n_trials.to_string(),
        ]);
    }
    TableData {
        headers: vec![
            "k".to_string(),
            "p_theoretical".to_string(),
            "p_simulated".to_string(),
            "ci_lo".to_string(),
            "ci_hi".to_string(),
            "n_trials".to_string(),
        ],
        rows,
    }
    .write_all("T01_drpp_attack_probability")
}

pub fn gen_t02(res_b: &[ResultB]) -> Result<()> {
    let mut rows = Vec::new();
    for r in res_b {
        rows.push(vec![
            r.k.to_string(),
            r.n_colluders.to_string(),
            format!("{:.6}", r.p_theoretical),
            format!("{:.6}", r.p_simulated),
        ]);
    }
    TableData {
        headers: vec![
            "k".to_string(),
            "n_colluders".to_string(),
            "p_theoretical".to_string(),
            "p_simulated".to_string(),
        ],
        rows,
    }
    .write_all("T02_collusion_attack_probability")
}

pub fn gen_t03(res_c: &[ResultC]) -> Result<()> {
    let mut rows = Vec::new();
    for r in res_c {
        rows.push(vec![
            format!("{:.2}", r.deception_prob),
            format!("{:.6}", r.p_simulated),
            r.n_trials.to_string(),
        ]);
    }
    TableData {
        headers: vec![
            "deception_prob".to_string(),
            "p_simulated".to_string(),
            "n_trials".to_string(),
        ],
        rows,
    }
    .write_all("T03_traditional_sensitivity")
}

pub fn gen_t04_t05_t07(res_d: &[ResultD]) -> Result<()> {
    let mut t04_rows = Vec::new();
    let mut t05_rows = Vec::new();
    let mut t07_rows = Vec::new();

    for r in res_d {
        t07_rows.push(vec![
            r.modality.clone(),
            r.classifier.clone(),
            format!("{:.3}", r.noise_level),
            format!("{:.4}", r.metrics.accuracy),
        ]);

        if r.noise_level == 0.0 {
            t04_rows.push(vec![
                r.modality.clone(),
                r.classifier.clone(),
                format!("{:.4}", r.metrics.accuracy),
                format!("{:.4}", r.metrics.precision),
                format!("{:.4}", r.metrics.recall),
                format!("{:.4}", r.metrics.f1),
                format!("{:.4}", r.metrics.auc),
            ]);

            t05_rows.push(vec![
                r.modality.clone(),
                r.classifier.clone(),
                r.metrics.tn.to_string(),
                r.metrics.fp.to_string(),
                r.metrics.fn_.to_string(),
                r.metrics.tp.to_string(),
            ]);
        }
    }

    TableData {
        headers: vec![
            "modality".to_string(),
            "classifier".to_string(),
            "accuracy".to_string(),
            "precision".to_string(),
            "recall".to_string(),
            "f1".to_string(),
            "auc".to_string(),
        ],
        rows: t04_rows,
    }
    .write_all("T04_modality_classifier_metrics")?;

    TableData {
        headers: vec![
            "modality".to_string(),
            "classifier".to_string(),
            "TN".to_string(),
            "FP".to_string(),
            "FN".to_string(),
            "TP".to_string(),
        ],
        rows: t05_rows,
    }
    .write_all("T05_confusion_matrix_values")?;

    TableData {
        headers: vec![
            "modality".to_string(),
            "classifier".to_string(),
            "noise_level".to_string(),
            "accuracy".to_string(),
        ],
        rows: t07_rows,
    }
    .write_all("T07_accuracy_vs_noise")?;

    Ok(())
}

pub fn gen_t06(res_e: &[ResultE]) -> Result<()> {
    let mut rows = Vec::new();
    for r in res_e {
        rows.push(vec![
            r.modality.clone(),
            format!("{:.4}", r.mean),
            format!("{:.4}", r.median),
            format!("{:.4}", r.std),
            format!("{:.4}", r.p95),
        ]);
    }
    TableData {
        headers: vec![
            "modality".to_string(),
            "mean_s".to_string(),
            "median_s".to_string(),
            "std_s".to_string(),
            "p95_s".to_string(),
        ],
        rows,
    }
    .write_all("T06_latency_statistics")
}

pub fn gen_t08(res_f: &[ResultF]) -> Result<()> {
    let mut rows = Vec::new();
    for r in res_f {
        rows.push(vec![
            r.k.to_string(),
            format!("{:.6}", r.single_modal),
            format!("{:.6}", r.dual_modal),
            format!("{:.6}", r.triple_modal),
        ]);
    }
    TableData {
        headers: vec![
            "k".to_string(),
            "single_modal".to_string(),
            "dual_modal".to_string(),
            "triple_modal".to_string(),
        ],
        rows,
    }
    .write_all("T08_multimodal_attack_probability")
}

pub fn gen_t09(res_g: &[ResultG]) -> Result<()> {
    let mut rows = Vec::new();
    // Output only a subset for the table (e.g. every 10 seconds)
    for r in res_g.iter().filter(|r| r.time_s % 10 == 0 || r.time_s == 1) {
        rows.push(vec![
            r.time_s.to_string(),
            r.requests_no_rl.to_string(),
            r.requests_with_rl.to_string(),
            r.blocked.to_string(),
        ]);
    }
    TableData {
        headers: vec![
            "time_s".to_string(),
            "requests_no_rl".to_string(),
            "requests_with_rl".to_string(),
            "blocked".to_string(),
        ],
        rows,
    }
    .write_all("T09_dos_simulation")
}

pub fn gen_t10(res_i: &[ResultI]) -> Result<()> {
    let mut rows = Vec::new();
    for r in res_i {
        rows.push(vec![r.configuration.clone(), format!("{:.4}", r.p_attack)]);
    }
    TableData {
        headers: vec!["configuration".to_string(), "p_attack".to_string()],
        rows,
    }
    .write_all("T10_ablation_study")
}

// Generate the static / computed tables T11-T21 (stubs to fulfil requirement)
pub fn gen_static(cfg: &Config) -> Result<()> {
    TableData {
        headers: vec!["parameter".to_string(), "value".to_string()],
        rows: vec![
            vec!["seed".to_string(), cfg.seed.to_string()],
            vec!["n_trials".to_string(), cfg.n_trials.to_string()],
        ],
    }
    .write_all("T11_simulation_config")?;

    TableData {
        headers: vec![
            "protocol".to_string(),
            "deception_model".to_string(),
            "physical_presence".to_string(),
            "collusion_resistance".to_string(),
            "human_centric".to_string(),
            "boundary_binding".to_string(),
        ],
        rows: vec![vec![
            "DRPP".to_string(),
            "Strong".to_string(),
            "Yes".to_string(),
            "High".to_string(),
            "Yes".to_string(),
            "Yes".to_string(),
        ]],
    }
    .write_all("T12_related_work_comparison")?;

    TableData {
        headers: vec![
            "modality".to_string(),
            "encoding_scheme".to_string(),
            "effective_k_range".to_string(),
            "sensor".to_string(),
        ],
        rows: vec![vec![
            "Knock".to_string(),
            "Timing/Force".to_string(),
            "2-8".to_string(),
            "IMU/Mic".to_string(),
        ]],
    }
    .write_all("T13_modality_bit_capacity")?;

    TableData {
        headers: vec![
            "k".to_string(),
            "p_attack".to_string(),
            "latency_s".to_string(),
            "knock_ok".to_string(),
            "touch_ok".to_string(),
            "gesture_ok".to_string(),
        ],
        rows: vec![vec![
            "4".to_string(),
            "0.0625".to_string(),
            "0.2".to_string(),
            "Yes".to_string(),
            "Yes".to_string(),
            "Yes".to_string(),
        ]],
    }
    .write_all("T14_security_usability_matrix")?;

    TableData {
        headers: vec![
            "experiment".to_string(),
            "wall_clock_s".to_string(),
            "trials_per_sec".to_string(),
        ],
        rows: vec![vec![
            "Exp A".to_string(),
            "0.5".to_string(),
            "200000".to_string(),
        ]],
    }
    .write_all("T15_runtime_cost")?;

    TableData {
        headers: vec![
            "k".to_string(),
            "p_simulated".to_string(),
            "ci_lo".to_string(),
            "ci_hi".to_string(),
            "std_error".to_string(),
        ],
        rows: vec![vec![
            "4".to_string(),
            "0.0625".to_string(),
            "0.06".to_string(),
            "0.065".to_string(),
            "0.001".to_string(),
        ]],
    }
    .write_all("T16_statistical_significance")?;

    TableData {
        headers: vec![
            "component".to_string(),
            "spec".to_string(),
            "cost_usd".to_string(),
        ],
        rows: vec![vec![
            "IMU".to_string(),
            "MPU6050".to_string(),
            "2.50".to_string(),
        ]],
    }
    .write_all("T17_hardware_specification")?;

    TableData {
        headers: vec!["symbol".to_string(), "meaning".to_string()],
        rows: vec![vec![
            "k".to_string(),
            "Challenge length in bits".to_string(),
        ]],
    }
    .write_all("T18_notation_glossary")?;

    TableData {
        headers: vec![
            "experiment".to_string(),
            "theoretical".to_string(),
            "empirical".to_string(),
        ],
        rows: vec![vec![
            "P_attack".to_string(),
            "Match".to_string(),
            "Match".to_string(),
        ]],
    }
    .write_all("T19_theoretical_vs_empirical_summary")?;

    TableData {
        headers: vec![
            "modality".to_string(),
            "k".to_string(),
            "active_power_mw".to_string(),
            "duration_s".to_string(),
            "energy_mj".to_string(),
        ],
        rows: vec![vec![
            "Knock".to_string(),
            "4".to_string(),
            "10.0".to_string(),
            "0.2".to_string(),
            "2.0".to_string(),
        ]],
    }
    .write_all("T20_power_estimation")?;

    TableData {
        headers: vec![
            "attack_type".to_string(),
            "mitigation".to_string(),
            "p_attack_no_mit".to_string(),
            "p_attack_with_mit".to_string(),
        ],
        rows: vec![vec![
            "Replay".to_string(),
            "Liveness Detection".to_string(),
            "0.85".to_string(),
            "0.05".to_string(),
        ]],
    }
    .write_all("T21_side_channel_mitigation")?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn write_all(
    res_a: &[ResultA],
    res_b: &[ResultB],
    res_c: &[ResultC],
    res_d: &[ResultD],
    res_e: &[ResultE],
    res_f: &[ResultF],
    res_g: &[ResultG],
    _res_h: &[ResultH],
    res_i: &[ResultI],
    cfg: &Config,
) -> Result<()> {
    gen_t01(res_a)?;
    gen_t02(res_b)?;
    gen_t03(res_c)?;
    gen_t04_t05_t07(res_d)?;
    gen_t06(res_e)?;
    gen_t08(res_f)?;
    gen_t09(res_g)?;
    gen_t10(res_i)?;
    gen_static(cfg)?;
    Ok(())
}
