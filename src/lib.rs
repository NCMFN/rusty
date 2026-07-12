use sha3::{Digest, Sha3_256};
use rand::Rng;
use rayon::prelude::*;

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

pub fn hash_r(r: &[i64]) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    for val in r {
        hasher.update(&val.to_le_bytes());
    }
    hasher.finalize().into()
}

pub fn sample_uniform<R: Rng>(rng: &mut R, n: usize) -> ZqVec {
    (0..n).map(|_| rng.gen_range(0..Q)).collect()
}

pub fn sample_error<R: Rng>(rng: &mut R, n: usize) -> ZqVec {
    (0..n).map(|_| {
        let u: f64 = rng.gen();
        let v: f64 = rng.gen();
        let z = (-2.0 * u.ln()).sqrt() * (2.0 * std::f64::consts::PI * v).cos();
        let sample = (z * SIGMA).round() as i64;
        let sample_mod = (sample % Q + Q) % Q;
        sample_mod
    }).collect()
}

pub fn encode(m: &[u8], n: usize) -> ZqVec {
    let digest = Sha3_256::digest(m);
    let half_q = Q / 2;
    let mut v = vec![0i64; n];
    for (i, byte) in digest.iter().enumerate() {
        for bit in 0..8 {
            let idx = i * 8 + bit;
            if idx >= n { return v; }
            if (byte >> bit) & 1 == 1 { v[idx] = half_q; }
        }
    }
    v
}

pub fn mat_vec_mul_fast(a: &[Vec<u16>], v: &[i64], out: &mut [i64]) {
    let n = a.len();
    out.par_iter_mut().enumerate().for_each(|(i, out_val)| {
        let mut acc: u32 = 0;
        let row = &a[i];
        let mut j = 0;
        while j + 4 <= n {
            acc += row[j] as u32 * v[j] as u32;
            acc += row[j+1] as u32 * v[j+1] as u32;
            acc += row[j+2] as u32 * v[j+2] as u32;
            acc += row[j+3] as u32 * v[j+3] as u32;
            if acc >= 1 << 30 { acc %= Q_U32; }
            j += 4;
        }
        while j < n {
            acc += row[j] as u32 * v[j] as u32;
            j += 1;
        }
        *out_val = (acc % Q_U32) as i64;
    });
}

pub fn vec_add_mod_fast(a: &[i64], b: &[i64], c: &[i64], out: &mut [i64]) {
    for i in 0..a.len() {
        out[i] = (a[i] + b[i] + c[i]) % Q;
    }
}

pub fn commit<R: Rng>(
    pp: &PublicParams,
    m: &[u8],
    rng: &mut R,
    buf_ar: &mut ZqVec,
    buf_c2: &mut ZqVec,
) -> (Commitment, OpeningHint) {
    let r = sample_uniform(rng, pp.n);
    let e = sample_error(rng, pp.n);
    let c1 = hash_r(&r);
    mat_vec_mul_fast(&pp.a, &r, buf_ar);
    let em = encode(m, pp.n);
    vec_add_mod_fast(buf_ar, &em, &e, buf_c2);
    (
        Commitment { c1, c2: buf_c2.clone() },
        OpeningHint { r, e, m: m.to_vec() }
    )
}

pub fn verify(pp: &PublicParams, c: &Commitment, hint: &OpeningHint) -> bool {
    if hash_r(&hint.r) != c.c1 { return false; }
    let mut ar = vec![0; pp.n];
    mat_vec_mul_fast(&pp.a, &hint.r, &mut ar);
    let em = encode(&hint.m, pp.n);
    let mut res = vec![0; pp.n];
    vec_add_mod_fast(&ar, &em, &hint.e, &mut res);
    res == c.c2
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_commit_verify_works() {
        let n = 256;
        let mut rng = thread_rng();
        let mut a = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                a[i][j] = rng.gen_range(0..Q) as u16;
            }
        }
        let pp = PublicParams { a, n };

        let msg = b"test message";
        let mut buf_ar = vec![0; n];
        let mut buf_c2 = vec![0; n];

        let (c, hint) = commit(&pp, msg, &mut rng, &mut buf_ar, &mut buf_c2);

        assert!(verify(&pp, &c, &hint));
    }
}
