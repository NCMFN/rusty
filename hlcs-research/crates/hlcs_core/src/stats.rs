use std::collections::VecDeque;

pub struct LatencyStats {
    pub mean_ms: f64,
    pub median_ms: f64,
    pub std_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub p95_ms: f64,
    pub p99_ms: f64,
    pub ci_lower_ms: f64,
    pub ci_upper_ms: f64,
}

pub fn calculate_stats(mut latencies: Vec<f64>) -> LatencyStats {
    if latencies.is_empty() {
        return LatencyStats {
            mean_ms: 0.0,
            median_ms: 0.0,
            std_ms: 0.0,
            min_ms: 0.0,
            max_ms: 0.0,
            p95_ms: 0.0,
            p99_ms: 0.0,
            ci_lower_ms: 0.0,
            ci_upper_ms: 0.0,
        };
    }

    let n = latencies.len() as f64;
    let sum: f64 = latencies.iter().sum();
    let mean_ms = sum / n;

    let variance: f64 = latencies
        .iter()
        .map(|value| {
            let diff = mean_ms - value;
            diff * diff
        })
        .sum::<f64>()
        / n;
    let std_ms = variance.sqrt();

    latencies.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let median_ms = latencies[(n * 0.5) as usize];
    let p95_ms = latencies[(n * 0.95) as usize];
    let p99_ms = latencies[(n * 0.99) as usize];
    let min_ms = latencies[0];
    let max_ms = latencies[latencies.len() - 1];

    // 95% CI for the mean: mean +/- 1.96 * (std / sqrt(n))
    let margin_of_error = 1.96 * (std_ms / n.sqrt());
    let ci_lower_ms = mean_ms - margin_of_error;
    let ci_upper_ms = mean_ms + margin_of_error;

    LatencyStats {
        mean_ms,
        median_ms,
        std_ms,
        min_ms,
        max_ms,
        p95_ms,
        p99_ms,
        ci_lower_ms,
        ci_upper_ms,
    }
}

pub struct RollingMean {
    window_size: usize,
    window: VecDeque<f64>,
    sum: f64,
}

impl RollingMean {
    pub fn new(window_size: usize) -> Self {
        Self {
            window_size,
            window: VecDeque::with_capacity(window_size),
            sum: 0.0,
        }
    }

    pub fn add(&mut self, value: f64) -> f64 {
        if self.window.len() == self.window_size {
            let removed = self.window.pop_front().unwrap();
            self.sum -= removed;
        }
        self.window.push_back(value);
        self.sum += value;
        self.mean()
    }

    pub fn mean(&self) -> f64 {
        if self.window.is_empty() {
            0.0
        } else {
            self.sum / self.window.len() as f64
        }
    }
}
