use rand::Rng;
use rand_distr::{Distribution, Normal};
use rayon::prelude::*;
use sha3::{Digest, Sha3_256};

pub const Q: i64 = 12289;
pub const Q_U32: u32 = 12289;
pub const SIGMA: f64 = 3.2;
pub type ZqVec = Vec<i64>;

pub struct PublicParams {
    pub a: Vec<Vec<u16>>,
    pub n: usize,
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

pub fn generate_matrix<R: Rng>(n: usize, rng: &mut R) -> Vec<Vec<u16>> {
    (0..n)
        .map(|_| (0..n).map(|_| rng.gen_range(0..Q_U32) as u16).collect())
        .collect()
}

pub fn encode(m: &[u8], n: usize) -> ZqVec {
    let mut hasher = Sha3_256::new();
    hasher.update(m);
    let digest = hasher.finalize();

    let mut out = vec![0; n];
    let mut byte_idx = 0;
    let mut bit_idx = 0;
    #[allow(clippy::needless_range_loop)]
    for i in 0..n {
        let bit = (digest[byte_idx] >> bit_idx) & 1;
        if bit == 1 {
            out[i] = Q / 2;
        }

        bit_idx += 1;
        if bit_idx == 8 {
            bit_idx = 0;
            byte_idx += 1;
            if byte_idx == 32 {
                byte_idx = 0;
            }
        }
    }
    out
}

pub fn mat_vec_mul_fast(a: &[Vec<u16>], v: &[i64], out: &mut [i64]) {
    let n = a.len();
    #[allow(clippy::needless_range_loop)]
    for i in 0..n {
        let mut sum: i64 = 0;
        let row = &a[i];
        for j in 0..n {
            sum += (row[j] as i64) * v[j];
        }
        out[i] = sum % Q;
    }
}

pub fn mat_vec_mul_parallel(a: &[Vec<u16>], v: &[i64], out: &mut [i64]) {
    let n = a.len();
    out.par_iter_mut().enumerate().for_each(|(i, val)| {
        let mut sum: i64 = 0;
        let row = &a[i];
        for j in 0..n {
            sum += (row[j] as i64) * v[j];
        }
        *val = sum % Q;
    });
}

pub fn sample_error<R: Rng>(n: usize, rng: &mut R) -> ZqVec {
    let normal = Normal::new(0.0, SIGMA).unwrap();
    (0..n)
        .map(|_| {
            let v = normal.sample(rng).round() as i64;
            ((v % Q) + Q) % Q
        })
        .collect()
}

pub fn sample_uniform<R: Rng>(n: usize, rng: &mut R) -> ZqVec {
    (0..n).map(|_| rng.gen_range(0..Q)).collect()
}

// --- HASH-ONLY BASELINE ---
pub fn commit_hash_only<R: Rng>(m: &[u8], n: usize, rng: &mut R) -> (Commitment, OpeningHint) {
    let r = sample_uniform(n, rng);
    let mut hasher = Sha3_256::new();
    let r_bytes: Vec<u8> = r.iter().flat_map(|x| x.to_le_bytes()).collect();
    hasher.update(&r_bytes);
    hasher.update(m);

    let mut c1 = [0u8; 32];
    c1.copy_from_slice(&hasher.finalize());

    (
        Commitment { c1, c2: vec![] },
        OpeningHint {
            r,
            e: vec![],
            m: m.to_vec(),
        },
    )
}

pub fn verify_hash_only(c: &Commitment, hint: &OpeningHint) -> bool {
    let mut hasher = Sha3_256::new();
    let r_bytes: Vec<u8> = hint.r.iter().flat_map(|x| x.to_le_bytes()).collect();
    hasher.update(&r_bytes);
    hasher.update(&hint.m);

    c.c1 == hasher.finalize().as_slice()
}

// --- LATTICE-ONLY BASELINE ---
pub fn commit_lattice_only<R: Rng>(
    pp: &PublicParams,
    m: &[u8],
    rng: &mut R,
    buf_ar: &mut ZqVec,
    buf_c2: &mut ZqVec,
) -> (Commitment, OpeningHint) {
    let r = sample_uniform(pp.n, rng);
    let e = sample_error(pp.n, rng);
    let encoded_m = encode(m, pp.n);

    if pp.n >= 768 {
        mat_vec_mul_parallel(&pp.a, &r, buf_ar);
    } else {
        mat_vec_mul_fast(&pp.a, &r, buf_ar);
    }

    for i in 0..pp.n {
        buf_c2[i] = (buf_ar[i] + e[i] + encoded_m[i]) % Q;
    }

    (
        Commitment {
            c1: [0u8; 32],
            c2: buf_c2.clone(),
        },
        OpeningHint {
            r,
            e,
            m: m.to_vec(),
        },
    )
}

pub fn verify_lattice_only(pp: &PublicParams, c: &Commitment, hint: &OpeningHint) -> bool {
    let encoded_m = encode(&hint.m, pp.n);
    let mut ar = vec![0; pp.n];
    if pp.n >= 768 {
        mat_vec_mul_parallel(&pp.a, &hint.r, &mut ar);
    } else {
        mat_vec_mul_fast(&pp.a, &hint.r, &mut ar);
    }

    for i in 0..pp.n {
        let expected = (ar[i] + hint.e[i] + encoded_m[i]) % Q;
        if c.c2[i] != expected {
            return false;
        }
    }
    true
}

// --- HYBRID SCHEME (HLCS-HFT) ---
pub fn commit_hybrid<R: Rng>(
    pp: &PublicParams,
    m: &[u8],
    rng: &mut R,
    buf_ar: &mut ZqVec,
    buf_c2: &mut ZqVec,
) -> (Commitment, OpeningHint) {
    let r = sample_uniform(pp.n, rng);
    let e = sample_error(pp.n, rng);
    let encoded_m = encode(m, pp.n);

    // c1 = H(r)
    let mut hasher = Sha3_256::new();
    let r_bytes: Vec<u8> = r.iter().flat_map(|x| x.to_le_bytes()).collect();
    hasher.update(&r_bytes);
    let mut c1 = [0u8; 32];
    c1.copy_from_slice(&hasher.finalize());

    if pp.n >= 768 {
        mat_vec_mul_parallel(&pp.a, &r, buf_ar);
    } else {
        mat_vec_mul_fast(&pp.a, &r, buf_ar);
    }

    for i in 0..pp.n {
        buf_c2[i] = (buf_ar[i] + e[i] + encoded_m[i]) % Q;
    }

    (
        Commitment {
            c1,
            c2: buf_c2.clone(),
        },
        OpeningHint {
            r,
            e,
            m: m.to_vec(),
        },
    )
}

pub fn verify_hybrid(pp: &PublicParams, c: &Commitment, hint: &OpeningHint) -> bool {
    // Check c1 = H(r)
    let mut hasher = Sha3_256::new();
    let r_bytes: Vec<u8> = hint.r.iter().flat_map(|x| x.to_le_bytes()).collect();
    hasher.update(&r_bytes);
    if c.c1 != hasher.finalize().as_slice() {
        return false;
    }

    let encoded_m = encode(&hint.m, pp.n);
    let mut ar = vec![0; pp.n];
    if pp.n >= 768 {
        mat_vec_mul_parallel(&pp.a, &hint.r, &mut ar);
    } else {
        mat_vec_mul_fast(&pp.a, &hint.r, &mut ar);
    }

    for i in 0..pp.n {
        let expected = (ar[i] + hint.e[i] + encoded_m[i]) % Q;
        if c.c2[i] != expected {
            return false;
        }
    }
    true
}

// --- FIAT-SHAMIR ZK LAYER ---
pub struct ZKProof {
    pub t1: ZqVec,
    pub sr: ZqVec,
    pub se: ZqVec,
}

pub fn prove_zk_1round<R: Rng>(
    pp: &PublicParams,
    c: &Commitment,
    hint: &OpeningHint,
    rng: &mut R,
) -> ZKProof {
    // 1. Prover samples y_r, y_e from Gaussian (or uniform in a larger range; for simplicity using uniform here)
    let yr = sample_uniform(pp.n, rng);
    let ye = sample_uniform(pp.n, rng);

    // 2. t1 = A * y_r + y_e
    let mut ayr = vec![0; pp.n];
    if pp.n >= 768 {
        mat_vec_mul_parallel(&pp.a, &yr, &mut ayr);
    } else {
        mat_vec_mul_fast(&pp.a, &yr, &mut ayr);
    }

    let mut t1 = vec![0; pp.n];
    for i in 0..pp.n {
        t1[i] = (ayr[i] + ye[i]) % Q;
    }

    // 3. alpha = H(t1 || c2) (challenge)
    let mut hasher = Sha3_256::new();
    let t1_bytes: Vec<u8> = t1.iter().flat_map(|x| x.to_le_bytes()).collect();
    let c2_bytes: Vec<u8> = c.c2.iter().flat_map(|x| x.to_le_bytes()).collect();
    hasher.update(&t1_bytes);
    hasher.update(&c2_bytes);
    let digest = hasher.finalize();
    // Use first few bytes as a small challenge alpha
    let alpha = (digest[0] as i64) % Q;

    // 4. sr = yr + alpha * r, se = ye + alpha * e
    let mut sr = vec![0; pp.n];
    let mut se = vec![0; pp.n];
    for i in 0..pp.n {
        sr[i] = (yr[i] + alpha * hint.r[i]) % Q;
        se[i] = (ye[i] + alpha * hint.e[i]) % Q;
    }

    ZKProof { t1, sr, se }
}

pub fn verify_zk_1round(pp: &PublicParams, c: &Commitment, m: &[u8], proof: &ZKProof) -> bool {
    let mut hasher = Sha3_256::new();
    let t1_bytes: Vec<u8> = proof.t1.iter().flat_map(|x| x.to_le_bytes()).collect();
    let c2_bytes: Vec<u8> = c.c2.iter().flat_map(|x| x.to_le_bytes()).collect();
    hasher.update(&t1_bytes);
    hasher.update(&c2_bytes);
    let digest = hasher.finalize();
    let alpha = (digest[0] as i64) % Q;

    // c2' = c2 - Enc(m) = A * r + e
    let encoded_m = encode(m, pp.n);
    let mut c2_prime = vec![0; pp.n];
    for i in 0..pp.n {
        c2_prime[i] = ((c.c2[i] - encoded_m[i]) % Q + Q) % Q;
    }

    // Check A * sr + se == t1 + alpha * c2'
    let mut asr = vec![0; pp.n];
    if pp.n >= 768 {
        mat_vec_mul_parallel(&pp.a, &proof.sr, &mut asr);
    } else {
        mat_vec_mul_fast(&pp.a, &proof.sr, &mut asr);
    }

    for i in 0..pp.n {
        let lhs = (asr[i] + proof.se[i]) % Q;
        let rhs = (proof.t1[i] + alpha * c2_prime[i]) % Q;
        if (lhs - rhs) % Q != 0 {
            return false;
        }
    }
    true
}

// Simulated 10-round aggregated settlement path
pub fn prove_zk_10round<R: Rng>(
    pp: &PublicParams,
    c: &Commitment,
    hint: &OpeningHint,
    rng: &mut R,
) -> Vec<ZKProof> {
    (0..10).map(|_| prove_zk_1round(pp, c, hint, rng)).collect()
}

pub fn verify_zk_10round(pp: &PublicParams, c: &Commitment, m: &[u8], proofs: &[ZKProof]) -> bool {
    if proofs.len() != 10 {
        return false;
    }
    proofs.iter().all(|p| verify_zk_1round(pp, c, m, p))
}
