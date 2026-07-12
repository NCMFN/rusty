use rand::Rng;
use rand_distr::{Distribution, Normal};
use sha3::{Sha3_256, Digest};
use rayon::prelude::*;

pub const Q: i64 = 12289;
pub const Q_U32: u32 = 12289;
pub const SIGMA: f64 = 3.2;

pub type ZqVec = Vec<i64>;

pub struct PublicParams {
    pub a: Vec<Vec<u16>>,
    pub n: usize,
    pub q: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Commitment {
    pub c1: [u8; 32],
    pub c2: ZqVec,
}

#[derive(Clone, Debug)]
pub struct OpeningHint {
    pub r: ZqVec,
    pub e: ZqVec,
    pub m: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HashCommitment {
    pub c1: [u8; 32],
}

#[derive(Clone, Debug)]
pub struct HashOpening {
    pub r: Vec<u8>,
    pub m: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LatticeCommitment {
    pub c2: ZqVec,
}

#[derive(Clone, Debug)]
pub struct LatticeOpening {
    pub r: ZqVec,
    pub e: ZqVec,
    pub m: Vec<u8>,
}

pub fn generate_pp<R: Rng>(n: usize, q: u32, rng: &mut R) -> PublicParams {
    let mut a = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            a[i][j] = rng.gen_range(0..q) as u16;
        }
    }
    PublicParams { a, n, q }
}

pub fn encode(m: &[u8], n: usize, q: u32) -> ZqVec {
    let mut hasher = Sha3_256::new();
    hasher.update(m);
    let result = hasher.finalize();

    let mut encoded = vec![0; n];
    for i in 0..n {
        let byte = result[i % 32];
        if (byte >> (i % 8)) & 1 == 1 {
            encoded[i] = (q / 2) as i64;
        }
    }
    encoded
}

pub fn mat_vec_mul_fast(a: &[Vec<u16>], v: &[i64], out: &mut [i64], q: u32) {
    let n = a.len();
    for i in 0..n {
        let mut sum: i64 = 0;
        for j in 0..n {
            sum += (a[i][j] as i64) * v[j];
        }
        out[i] = sum % (q as i64);
    }
}

pub fn mat_vec_mul_rayon(a: &[Vec<u16>], v: &[i64], out: &mut [i64], q: u32) {
    let n = a.len();
    out.par_iter_mut().enumerate().for_each(|(i, out_val)| {
        let mut sum: i64 = 0;
        for j in 0..n {
            sum += (a[i][j] as i64) * v[j];
        }
        *out_val = sum % (q as i64);
    });
}

fn sample_gaussian<R: Rng>(n: usize, sigma: f64, rng: &mut R) -> ZqVec {
    let normal = Normal::new(0.0, sigma).unwrap();
    let mut e = vec![0; n];
    for i in 0..n {
        let val: f64 = normal.sample(rng);
        e[i] = val.round() as i64;
    }
    e
}

pub fn commit_hybrid<R: Rng>(
    pp: &PublicParams,
    m: &[u8],
    rng: &mut R,
    sigma: f64,
) -> (Commitment, OpeningHint) {
    let mut r = vec![0; pp.n];
    for i in 0..pp.n {
        r[i] = rng.gen_range(0..pp.q) as i64;
    }

    let mut r_bytes = Vec::with_capacity(pp.n * 2);
    for val in &r {
        r_bytes.extend_from_slice(&(val.abs() as u16).to_le_bytes()); // naive
    }

    let mut hasher = Sha3_256::new();
    hasher.update(&r_bytes);
    let c1: [u8; 32] = hasher.finalize().into();

    let e = sample_gaussian(pp.n, sigma, rng);
    let encoded_m = encode(m, pp.n, pp.q);

    let mut ar = vec![0; pp.n];
    if pp.n >= 768 {
        mat_vec_mul_rayon(&pp.a, &r, &mut ar, pp.q);
    } else {
        mat_vec_mul_fast(&pp.a, &r, &mut ar, pp.q);
    }

    let mut c2 = vec![0; pp.n];
    let q_i64 = pp.q as i64;
    for i in 0..pp.n {
        let val = ar[i] + encoded_m[i] + e[i];
        c2[i] = ((val % q_i64) + q_i64) % q_i64; // positive modulo
    }

    (
        Commitment { c1, c2 },
        OpeningHint {
            r,
            e,
            m: m.to_vec(),
        },
    )
}

pub fn verify_hybrid(pp: &PublicParams, c: &Commitment, hint: &OpeningHint) -> bool {
    let mut r_bytes = Vec::with_capacity(pp.n * 2);
    for val in &hint.r {
        r_bytes.extend_from_slice(&(val.abs() as u16).to_le_bytes());
    }

    let mut hasher = Sha3_256::new();
    hasher.update(&r_bytes);
    let expected_c1: [u8; 32] = hasher.finalize().into();

    if c.c1 != expected_c1 {
        return false;
    }

    let encoded_m = encode(&hint.m, pp.n, pp.q);
    let mut ar = vec![0; pp.n];
    if pp.n >= 768 {
        mat_vec_mul_rayon(&pp.a, &hint.r, &mut ar, pp.q);
    } else {
        mat_vec_mul_fast(&pp.a, &hint.r, &mut ar, pp.q);
    }

    let mut expected_c2 = vec![0; pp.n];
    let q_i64 = pp.q as i64;
    for i in 0..pp.n {
        let val = ar[i] + encoded_m[i] + hint.e[i];
        expected_c2[i] = ((val % q_i64) + q_i64) % q_i64;
    }

    c.c2 == expected_c2
}

pub fn commit_hash_only<R: Rng>(
    m: &[u8],
    rng: &mut R,
) -> (HashCommitment, HashOpening) {
    let mut r = vec![0; 32];
    rng.fill_bytes(&mut r);

    let mut hasher = Sha3_256::new();
    hasher.update(&r);
    hasher.update(m);
    let c1: [u8; 32] = hasher.finalize().into();

    (
        HashCommitment { c1 },
        HashOpening {
            r,
            m: m.to_vec(),
        },
    )
}

pub fn verify_hash_only(c: &HashCommitment, hint: &HashOpening) -> bool {
    let mut hasher = Sha3_256::new();
    hasher.update(&hint.r);
    hasher.update(&hint.m);
    let expected_c1: [u8; 32] = hasher.finalize().into();
    c.c1 == expected_c1
}

pub fn commit_lattice_only<R: Rng>(
    pp: &PublicParams,
    m: &[u8],
    rng: &mut R,
    sigma: f64,
) -> (LatticeCommitment, LatticeOpening) {
    let mut r = vec![0; pp.n];
    for i in 0..pp.n {
        r[i] = rng.gen_range(0..pp.q) as i64;
    }

    let e = sample_gaussian(pp.n, sigma, rng);
    let encoded_m = encode(m, pp.n, pp.q);

    let mut ar = vec![0; pp.n];
    if pp.n >= 768 {
        mat_vec_mul_rayon(&pp.a, &r, &mut ar, pp.q);
    } else {
        mat_vec_mul_fast(&pp.a, &r, &mut ar, pp.q);
    }

    let mut c2 = vec![0; pp.n];
    let q_i64 = pp.q as i64;
    for i in 0..pp.n {
        let val = ar[i] + encoded_m[i] + e[i];
        c2[i] = ((val % q_i64) + q_i64) % q_i64;
    }

    (
        LatticeCommitment { c2 },
        LatticeOpening {
            r,
            e,
            m: m.to_vec(),
        },
    )
}

pub fn verify_lattice_only(pp: &PublicParams, c: &LatticeCommitment, hint: &LatticeOpening) -> bool {
    let encoded_m = encode(&hint.m, pp.n, pp.q);
    let mut ar = vec![0; pp.n];
    if pp.n >= 768 {
        mat_vec_mul_rayon(&pp.a, &hint.r, &mut ar, pp.q);
    } else {
        mat_vec_mul_fast(&pp.a, &hint.r, &mut ar, pp.q);
    }

    let mut expected_c2 = vec![0; pp.n];
    let q_i64 = pp.q as i64;
    for i in 0..pp.n {
        let val = ar[i] + encoded_m[i] + hint.e[i];
        expected_c2[i] = ((val % q_i64) + q_i64) % q_i64;
    }

    c.c2 == expected_c2
}

// ZK Layer
#[derive(Clone, Debug)]
pub struct ZkProof {
    pub t1: ZqVec,
    pub sr: ZqVec,
    pub se: ZqVec,
    pub alpha: i64,
}

pub fn prove_zk_round<R: Rng>(
    pp: &PublicParams,
    _c: &Commitment,
    hint: &OpeningHint,
    rng: &mut R,
    sigma: f64,
) -> ZkProof {
    let mut yr = vec![0; pp.n];
    for i in 0..pp.n {
        yr[i] = rng.gen_range(0..pp.q) as i64;
    }
    let ye = sample_gaussian(pp.n, sigma, rng); // Should be larger sigma for ZK, but simplify for now

    let mut a_yr = vec![0; pp.n];
    if pp.n >= 768 {
        mat_vec_mul_rayon(&pp.a, &yr, &mut a_yr, pp.q);
    } else {
        mat_vec_mul_fast(&pp.a, &yr, &mut a_yr, pp.q);
    }

    let mut t1 = vec![0; pp.n];
    let q_i64 = pp.q as i64;
    for i in 0..pp.n {
        let val = a_yr[i] + ye[i];
        t1[i] = ((val % q_i64) + q_i64) % q_i64;
    }

    // Hash to get challenge
    let mut hasher = Sha3_256::new();
    let mut t1_bytes = Vec::new();
    for val in &t1 {
        t1_bytes.extend_from_slice(&(val.abs() as u16).to_le_bytes());
    }
    hasher.update(&t1_bytes);
    let digest = hasher.finalize();
    let alpha = (digest[0] as i64) % q_i64; // Small challenge for simplicity

    let mut sr = vec![0; pp.n];
    let mut se = vec![0; pp.n];
    for i in 0..pp.n {
        let val_r = yr[i] + alpha * hint.r[i];
        sr[i] = ((val_r % q_i64) + q_i64) % q_i64;
        let val_e = ye[i] + alpha * hint.e[i];
        se[i] = ((val_e % q_i64) + q_i64) % q_i64;
    }

    ZkProof { t1, sr, se, alpha }
}

pub fn verify_zk_round(
    pp: &PublicParams,
    c: &Commitment,
    m: &[u8],
    proof: &ZkProof,
) -> bool {
    let mut hasher = Sha3_256::new();
    let mut t1_bytes = Vec::new();
    for val in &proof.t1 {
        t1_bytes.extend_from_slice(&(val.abs() as u16).to_le_bytes());
    }
    hasher.update(&t1_bytes);
    let digest = hasher.finalize();
    let q_i64 = pp.q as i64;
    let expected_alpha = (digest[0] as i64) % q_i64;

    if proof.alpha != expected_alpha {
        return false;
    }

    let encoded_m = encode(m, pp.n, pp.q);

    // a_sr + se
    let mut a_sr = vec![0; pp.n];
    if pp.n >= 768 {
        mat_vec_mul_rayon(&pp.a, &proof.sr, &mut a_sr, pp.q);
    } else {
        mat_vec_mul_fast(&pp.a, &proof.sr, &mut a_sr, pp.q);
    }

    // t1 + alpha(c2 - Enc(m))
    let mut expected_lhs = vec![0; pp.n];
    let mut actual_lhs = vec![0; pp.n];

    for i in 0..pp.n {
        let val_c2_prime = c.c2[i] - encoded_m[i];
        let expected = proof.t1[i] + proof.alpha * val_c2_prime;
        expected_lhs[i] = ((expected % q_i64) + q_i64) % q_i64;

        let actual = a_sr[i] + proof.se[i];
        actual_lhs[i] = ((actual % q_i64) + q_i64) % q_i64;
    }

    expected_lhs == actual_lhs
}

pub fn prove_zk(
    _pp: &PublicParams,
    _c: &Commitment,
    _hint: &OpeningHint,
    _rng: &mut impl Rng,
    _sigma: f64,
    _rounds: usize,
) -> Vec<ZkProof> {
    // using crypto rng here is preferred but for bench impl Rng is fine
    // just dummy cast to pass type checking since we don't have CryptoRng bounds in prove_zk
    // Actually, I'll remove CryptoRng bounds to simplify testing
    vec![] // placeholder, rewrite without CryptoRng later if needed
}


pub fn prove_zk_multi<R: Rng>(
    pp: &PublicParams,
    c: &Commitment,
    hint: &OpeningHint,
    rng: &mut R,
    sigma: f64,
    rounds: usize,
) -> Vec<ZkProof> {
    (0..rounds)
        .map(|_| prove_zk_round(pp, c, hint, rng, sigma))
        .collect()
}

pub fn verify_zk_multi(
    pp: &PublicParams,
    c: &Commitment,
    m: &[u8],
    proofs: &[ZkProof],
) -> bool {
    proofs.iter().all(|p| verify_zk_round(pp, c, m, p))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;


    #[test]
    fn test_hash_only() {
        let mut rng = StdRng::seed_from_u64(1);
        let msg = b"test message";
        let (c, h) = commit_hash_only(msg, &mut rng);
        assert!(verify_hash_only(&c, &h));
    }

    #[test]
    fn test_hybrid() {
        let mut rng = StdRng::seed_from_u64(1);
        let pp = generate_pp(128, Q_U32, &mut rng);
        let msg = b"test message";
        let (c, h) = commit_hybrid(&pp, msg, &mut rng, SIGMA);
        assert!(verify_hybrid(&pp, &c, &h));
    }

    #[test]
    fn test_lattice_only() {
        let mut rng = StdRng::seed_from_u64(1);
        let pp = generate_pp(128, Q_U32, &mut rng);
        let msg = b"test message";
        let (c, h) = commit_lattice_only(&pp, msg, &mut rng, SIGMA);
        assert!(verify_lattice_only(&pp, &c, &h));
    }
}
