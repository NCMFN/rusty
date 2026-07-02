/// Calculates the Wilson score interval for binomial proportions.
/// Returns (lower_bound, upper_bound).
pub fn confidence_interval(successes: u64, n: u64, z: f64) -> (f64, f64) {
    if n == 0 {
        return (0.0, 0.0);
    }

    let p = successes as f64 / n as f64;
    let n_f64 = n as f64;

    let denominator = 1.0 + z * z / n_f64;
    let center = p + z * z / (2.0 * n_f64);

    let spread = z * (p * (1.0 - p) / n_f64 + z * z / (4.0 * n_f64 * n_f64)).sqrt();

    let lower = (center - spread) / denominator;
    let upper = (center + spread) / denominator;

    (lower.max(0.0), upper.min(1.0))
}

/// Standard 95% confidence interval (z = 1.96).
pub fn ci_95(successes: u64, n: u64) -> (f64, f64) {
    confidence_interval(successes, n, 1.96)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confidence_interval() {
        // As n grows, width should shrink
        let p = 0.5;
        let mut prev_width = f64::MAX;

        for n in [100, 1000, 10000] {
            let successes = (n as f64 * p) as u64;
            let (lo, hi) = ci_95(successes, n);
            let width = hi - lo;
            assert!(width < prev_width);
            prev_width = width;
        }
    }
}
