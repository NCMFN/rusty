use crate::lwe::{
    encode, mat_vec_mul_mod, sample_discrete_gaussian, vec_add_mod, vec_add_noise_mod, PublicParams,
};
use rand::rngs::StdRng;
use rand::Rng;
use sha3::{Digest, Sha3_256};

/// The hybrid commitment scheme Π_hyb.
/// C1 = H(r)
/// C2 = Ar + Encode(m) + e (mod q)
#[derive(Clone, Debug, PartialEq)]
pub struct HybridCommitment {
    pub c1: [u8; 32],
    pub c2: Vec<u32>,
}

#[derive(Clone, Debug)]
pub struct HybridOpening {
    pub message: Vec<u8>,
    pub r: Vec<u32>,
    pub e: Vec<i64>,
}

/// Helper to hash a vector of u32 (the random vector r)
fn hash_u32_vec(vec: &[u32]) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    for val in vec {
        hasher.update(&val.to_le_bytes());
    }
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}

/// Algorithm 1: HybridCommit(pp, m)
pub fn hybrid_commit(
    pp: &PublicParams,
    message: &[u8],
    sigma: f64,
    rng: &mut StdRng,
) -> (HybridCommitment, HybridOpening) {
    let mut r = vec![0; pp.n];
    for i in 0..pp.n {
        r[i] = rng.gen_range(0..pp.q);
    }

    let e = sample_discrete_gaussian(pp.n, sigma, rng);
    let encoded_m = encode(message, pp.n, pp.q);

    // C1 = H(r)
    let c1 = hash_u32_vec(&r);

    // C2 = Ar + Encode(m) + e (mod q)
    let ar = mat_vec_mul_mod(&pp.a, &r, pp.q);
    let ar_plus_m = vec_add_mod(&ar, &encoded_m, pp.q);
    let c2 = vec_add_noise_mod(&ar_plus_m, &e, pp.q);

    (
        HybridCommitment { c1, c2 },
        HybridOpening {
            message: message.to_vec(),
            r,
            e,
        },
    )
}

/// Algorithm 2: Verify(pp, C, m, r, e)
pub fn hybrid_verify(
    pp: &PublicParams,
    commitment: &HybridCommitment,
    opening: &HybridOpening,
) -> bool {
    // Fast path: short-circuit if hash check fails
    let expected_c1 = hash_u32_vec(&opening.r);
    if commitment.c1 != expected_c1 {
        return false;
    }

    // Lattice check
    let encoded_m = encode(&opening.message, pp.n, pp.q);
    let ar = mat_vec_mul_mod(&pp.a, &opening.r, pp.q);
    let ar_plus_m = vec_add_mod(&ar, &encoded_m, pp.q);
    let expected_c2 = vec_add_noise_mod(&ar_plus_m, &opening.e, pp.q);

    commitment.c2 == expected_c2
}
