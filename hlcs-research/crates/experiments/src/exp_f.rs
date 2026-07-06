use hlcs_core::RollingMean;

pub struct ExpFResult {
    pub latencies: Vec<f64>,
    pub rolling_means: Vec<f64>,
    pub threshold_ms: f64,
}

pub fn run(latencies: &[f64], window_size: usize, threshold_ms: f64) -> ExpFResult {
    let mut rm = RollingMean::new(window_size);
    let mut rolling_means = Vec::with_capacity(latencies.len());

    for &lat in latencies {
        rolling_means.push(rm.add(lat));
    }

    ExpFResult {
        latencies: latencies.to_vec(),
        rolling_means,
        threshold_ms,
    }
}
