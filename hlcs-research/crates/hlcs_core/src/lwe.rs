use rand::rngs::StdRng;
use rand::Rng;
use rand_distr::{Distribution, Normal};

/// Samples a random matrix A in Z_q^{n x n}.
pub fn sample_matrix_a(n: usize, q: u32, rng: &mut StdRng) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            matrix[i][j] = rng.gen_range(0..q);
        }
    }
    matrix
}

/// Samples a vector from a discrete Gaussian distribution approximated by rounding
/// draws from a Normal distribution with mean 0 and std dev sigma.
/// The result is reduced modulo q (but represented as i64 for signed operations before reduction).
/// Limitation: This is an approximation and not a true discrete Gaussian sampler (e.g. rejection sampling),
/// which is sufficient for evaluation purposes but theoretically distinguishable.
pub fn sample_discrete_gaussian(n: usize, sigma: f64, rng: &mut StdRng) -> Vec<i64> {
    let normal = Normal::new(0.0, sigma).unwrap();
    let mut result = vec![0; n];
    for i in 0..n {
        let sample = normal.sample(rng);
        result[i] = sample.round() as i64;
    }
    result
}

/// Deterministically encodes a byte slice into an element of Z_q^n.
/// This implementation uses a simple block expansion: each byte is placed
/// in an element of the vector. If the message is shorter than n, it is padded with 0.
/// If it is longer, it panics (for the purpose of this simulation, messages should fit).
pub fn encode(message: &[u8], n: usize, _q: u32) -> Vec<u32> {
    assert!(message.len() <= n, "Message length exceeds n");
    let mut result = vec![0; n];
    for i in 0..message.len() {
        result[i] = message[i] as u32;
    }
    result
}

/// Multiplies matrix A by vector r modulo q.
pub fn mat_vec_mul_mod(a: &[Vec<u32>], r: &[u32], q: u32) -> Vec<u32> {
    let n = a.len();
    let mut result = vec![0; n];
    for i in 0..n {
        let mut sum: u64 = 0;
        for j in 0..n {
            sum += (a[i][j] as u64) * (r[j] as u64);
        }
        result[i] = (sum % (q as u64)) as u32;
    }
    result
}

/// Adds two vectors modulo q.
pub fn vec_add_mod(a: &[u32], b: &[u32], q: u32) -> Vec<u32> {
    let n = a.len();
    let mut result = vec![0; n];
    for i in 0..n {
        result[i] = (a[i] + b[i]) % q;
    }
    result
}

/// Adds a vector of u32 and a vector of i64 (noise) modulo q.
pub fn vec_add_noise_mod(a: &[u32], noise: &[i64], q: u32) -> Vec<u32> {
    let n = a.len();
    let mut result = vec![0; n];
    for i in 0..n {
        let mut val = (a[i] as i64) + noise[i];
        val = val % (q as i64);
        if val < 0 {
            val += q as i64;
        }
        result[i] = val as u32;
    }
    result
}

/// Public parameters for the commitment schemes.
#[derive(Clone)]
pub struct PublicParams {
    pub n: usize,
    pub q: u32,
    pub a: Vec<Vec<u32>>,
}

impl PublicParams {
    pub fn generate(n: usize, q: u32, rng: &mut StdRng) -> Self {
        Self {
            n,
            q,
            a: sample_matrix_a(n, q, rng),
        }
    }
}
