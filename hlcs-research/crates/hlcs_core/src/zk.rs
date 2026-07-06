use crate::hybrid_commit::HybridCommitment;
use crate::lwe::{
    encode, mat_vec_mul_mod, sample_discrete_gaussian, vec_add_noise_mod, PublicParams,
};
use rand::rngs::StdRng;
use rand::Rng;
use sha3::{Digest, Sha3_256};

#[derive(Clone, Debug)]
pub struct ZkProof {
    pub c_bar: Vec<u32>,
    pub z: Vec<u32>,
    pub f: Vec<i64>,
}

fn hash_challenge(c: &HybridCommitment, c_bar: &[u32], q: u32) -> u32 {
    let mut hasher = Sha3_256::new();
    hasher.update(&c.c1);
    for val in &c.c2 {
        hasher.update(&val.to_le_bytes());
    }
    for val in c_bar {
        hasher.update(&val.to_le_bytes());
    }
    let result = hasher.finalize();
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&result[0..4]);
    u32::from_le_bytes(bytes) % q // Challenge space is Z_q to make soundness error negligible
}

/// Algorithm 3: Zero-Knowledge Proof (Sigma-protocol, Fiat-Shamir NIZK)
pub fn zk_prove(
    pp: &PublicParams,
    commitment: &HybridCommitment,
    _message: &[u8],
    r: &[u32],
    e: &[i64],
    sigma_prime: f64,
    rng: &mut StdRng,
) -> ZkProof {
    // 1. Prover sends C_bar = A * r_bar + e_bar
    let mut r_bar = vec![0; pp.n];
    for i in 0..pp.n {
        r_bar[i] = rng.gen_range(0..pp.q);
    }
    let e_bar = sample_discrete_gaussian(pp.n, sigma_prime, rng);

    let a_r_bar = mat_vec_mul_mod(&pp.a, &r_bar, pp.q);
    let c_bar = vec_add_noise_mod(&a_r_bar, &e_bar, pp.q);

    // 2. Verifier challenges c = H(C, C_bar) non-interactively
    let c_challenge = hash_challenge(commitment, &c_bar, pp.q);

    // 3. Prover responds z = r_bar + c * r, f = e_bar + c * e
    let mut z = vec![0; pp.n];
    let mut f = vec![0; pp.n];

    for i in 0..pp.n {
        let cr_i = (c_challenge as u64 * r[i] as u64) % (pp.q as u64);
        z[i] = (r_bar[i] + cr_i as u32) % pp.q;
        f[i] = e_bar[i] + (c_challenge as i64 * e[i]);
    }

    ZkProof { c_bar, z, f }
}

pub fn zk_verify(
    pp: &PublicParams,
    commitment: &HybridCommitment,
    proof: &ZkProof,
    message: &[u8],
) -> bool {
    let c_challenge = hash_challenge(commitment, &proof.c_bar, pp.q);

    // Az + f
    let az = mat_vec_mul_mod(&pp.a, &proof.z, pp.q);
    let lhs = vec_add_noise_mod(&az, &proof.f, pp.q);

    // C_bar + c * C2 - c * Encode(m)
    let encoded_m = encode(message, pp.n, pp.q);

    let mut c_c2 = vec![0; pp.n];
    for i in 0..pp.n {
        c_c2[i] = ((c_challenge as u64 * commitment.c2[i] as u64) % (pp.q as u64)) as u32;
    }

    let mut c_m = vec![0; pp.n];
    for i in 0..pp.n {
        c_m[i] = ((c_challenge as u64 * encoded_m[i] as u64) % (pp.q as u64)) as u32;
    }

    let mut rhs = vec![0; pp.n];
    for i in 0..pp.n {
        let sum = (proof.c_bar[i] + c_c2[i]) % pp.q;
        let diff = (sum as i64 - c_m[i] as i64) % (pp.q as i64);
        let mut final_val = diff;
        if final_val < 0 {
            final_val += pp.q as i64;
        }
        rhs[i] = final_val as u32;
    }

    lhs == rhs
}
