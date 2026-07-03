use statrs::distribution::{ContinuousCDF, Normal};

pub struct LasMargin {
    pub grover_time_log2: f64,
    pub lattice_time_log2: f64,
    pub tau_log2: f64,
    pub margin_orders_of_magnitude: f64,
}

/// Computes the collision term q_H^2 / 2^n from Theorem V.3
pub fn binding_bound(q_hash_queries: u64, n: usize) -> f64 {
    let qh_sq = (q_hash_queries as f64).powi(2);
    let two_n = 2.0_f64.powi(n as i32);
    qh_sq / two_n
}

/// Computes the LAS margin as described in Theorem V.4
pub fn las_margin(n: usize, tau_ms: f64) -> LasMargin {
    // Grover time complexity for n-bit hash is O(2^(n/2))
    let grover_time_log2 = (n as f64) / 2.0;

    // Lattice attack time complexity for LWE is assumed Omega(2^n) depending on parameters.
    // For n=512, best known attacks take > 2^512 operations per paper.
    // We'll use n as the log2 complexity for simplicity as per the paper's qualitative argument.
    let lattice_time_log2 = n as f64;

    // Estimate operations per millisecond for the adversary.
    // Say an adversary can do 2^40 operations per millisecond (extremely generous).
    let ops_per_ms_log2 = 40.0;
    let tau_log2 = ops_per_ms_log2 + tau_ms.log2();

    let min_attack_log2 = grover_time_log2.min(lattice_time_log2);
    let margin_log2 = min_attack_log2 - tau_log2;
    // log10(2^x) = x * log10(2)
    let margin_orders_of_magnitude = margin_log2 * 2.0_f64.log10();

    LasMargin {
        grover_time_log2,
        lattice_time_log2,
        tau_log2,
        margin_orders_of_magnitude,
    }
}

/// Reproduces the Section VII-E calculation for decryption failure bound
pub fn decryption_failure_bound(_sigma: f64, n: usize, b_over_sigma: f64) -> f64 {
    // Standard normal CDF since we're using multiples of sigma
    let normal = Normal::new(0.0, 1.0).unwrap();
    // P(|X| > B) = 2 * Phi(-B/sigma)
    let per_coordinate_bound = 2.0 * normal.cdf(-b_over_sigma);

    // Union bound over n coordinates: 1 - (1 - p)^n, approximated as n * p for small p
    // The paper gives exactly the union bound. We'll use the exact formula for probability of at least one failure.
    let per_commitment_bound = 1.0 - (1.0 - per_coordinate_bound).powi(n as i32);

    per_commitment_bound
}

pub fn decryption_failure_bound_per_coord(_sigma: f64, b_over_sigma: f64) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    2.0 * normal.cdf(-b_over_sigma)
}
