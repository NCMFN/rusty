use crate::lwe::{
    encode, mat_vec_mul_mod, sample_discrete_gaussian, vec_add_mod, vec_add_noise_mod, PublicParams,
};
use rand::rngs::StdRng;
use rand::Rng;

/// Lattice-only commitment scheme baseline.
/// C = Ar + Encode(m) + e (mod q)
#[derive(Clone, Debug, PartialEq)]
pub struct LatticeCommitment {
    pub c2: Vec<u32>,
}

#[derive(Clone, Debug)]
pub struct LatticeOpening {
    pub message: Vec<u8>,
    pub r: Vec<u32>,
    pub e: Vec<i64>,
}

pub fn lattice_commit(
    pp: &PublicParams,
    message: &[u8],
    sigma: f64,
    rng: &mut StdRng,
) -> (LatticeCommitment, LatticeOpening) {
    let mut r = vec![0; pp.n];
    for i in 0..pp.n {
        r[i] = rng.gen_range(0..pp.q);
    }

    let e = sample_discrete_gaussian(pp.n, sigma, rng);
    let encoded_m = encode(message, pp.n, pp.q);

    let ar = mat_vec_mul_mod(&pp.a, &r, pp.q);
    let ar_plus_m = vec_add_mod(&ar, &encoded_m, pp.q);
    let c2 = vec_add_noise_mod(&ar_plus_m, &e, pp.q);

    (
        LatticeCommitment { c2 },
        LatticeOpening {
            message: message.to_vec(),
            r,
            e,
        },
    )
}

pub fn lattice_verify(
    pp: &PublicParams,
    commitment: &LatticeCommitment,
    opening: &LatticeOpening,
) -> bool {
    let encoded_m = encode(&opening.message, pp.n, pp.q);
    let ar = mat_vec_mul_mod(&pp.a, &opening.r, pp.q);
    let ar_plus_m = vec_add_mod(&ar, &encoded_m, pp.q);
    let expected_c2 = vec_add_noise_mod(&ar_plus_m, &opening.e, pp.q);

    commitment.c2 == expected_c2
}
