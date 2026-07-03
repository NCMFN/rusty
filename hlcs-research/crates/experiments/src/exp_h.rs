use hlcs_core::{decryption_failure_bound, decryption_failure_bound_per_coord};

pub struct FailurePoint {
    pub b_over_sigma: f64,
    pub per_coordinate_bound: f64,
    pub per_commitment_bound: f64,
}

pub struct ExpHResult {
    pub data: Vec<FailurePoint>,
}

pub fn run(multiples: &[f64], n: usize, sigma: f64) -> ExpHResult {
    let mut data = Vec::with_capacity(multiples.len());

    for &b_over_sigma in multiples {
        let per_coordinate = decryption_failure_bound_per_coord(sigma, b_over_sigma);
        let per_commitment = decryption_failure_bound(sigma, n, b_over_sigma);

        data.push(FailurePoint {
            b_over_sigma,
            per_coordinate_bound: per_coordinate,
            per_commitment_bound: per_commitment,
        });
    }

    ExpHResult { data }
}
