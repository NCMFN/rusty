use hmac::{Hmac, Mac};
use rand::rngs::StdRng;
use rand::Rng;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Generates a random k-bit challenge.
pub fn generate_challenge(k: u32, rng: &mut StdRng) -> u32 {
    if k == 0 {
        return 0;
    }
    if k >= 32 {
        return rng.gen::<u32>();
    }
    let mask = (1u32 << k) - 1;
    rng.gen::<u32>() & mask
}

/// Computes the expected k-bit response using HMAC-SHA256 as the PRF.
pub fn compute_response(challenge: u32, secret: &[u8], k: u32) -> u32 {
    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC can take key of any size");
    mac.update(&challenge.to_le_bytes());
    let result = mac.finalize();
    let bytes = result.into_bytes();

    let mut val = [0u8; 4];
    val.copy_from_slice(&bytes[0..4]);
    let full_response = u32::from_le_bytes(val);

    if k == 0 {
        return 0;
    }
    if k >= 32 {
        return full_response;
    }

    let mask = (1u32 << k) - 1;
    full_response & mask
}

/// Verifies if a given response matches the expected one.
pub fn verify(challenge: u32, response: u32, secret: &[u8], k: u32) -> bool {
    let expected = compute_response(challenge, secret, k);
    expected == response
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn test_prf_deterministic() {
        let secret = b"test_secret";
        let k = 10;
        let challenge = 42;
        let r1 = compute_response(challenge, secret, k);
        let r2 = compute_response(challenge, secret, k);
        assert_eq!(r1, r2, "PRF must be deterministic");
    }

    #[test]
    fn test_prf_range() {
        let secret = b"test_secret";
        for k in 1..=20 {
            let challenge = 12345;
            let r = compute_response(challenge, secret, k);
            assert!(r < (1 << k), "Result must be < 2^k");
        }
    }

    #[test]
    fn test_verify_correct() {
        let secret = b"test_secret";
        let k = 12;
        let challenge = 999;
        let r = compute_response(challenge, secret, k);
        assert!(verify(challenge, r, secret, k), "Honest prover should pass");
    }

    #[test]
    fn test_verify_wrong() {
        let secret = b"test_secret";
        let k = 8;
        let challenge = 123;
        let _expected = compute_response(challenge, secret, k);

        let mut rng = StdRng::seed_from_u64(42);
        let mut failures = 0;
        let trials = 10000;
        for _ in 0..trials {
            let guess = generate_challenge(k, &mut rng);
            if !verify(challenge, guess, secret, k) {
                failures += 1;
            }
        }

        let fail_rate = failures as f64 / trials as f64;
        let expected_fail_rate = 1.0 - (1.0 / 256.0); // 1 - 2^-8

        // Allow a small statistical variance
        assert!(fail_rate >= expected_fail_rate - 0.02);
    }
}
