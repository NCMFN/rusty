pub mod hash_commit;
pub mod hybrid_commit;
pub mod lattice_commit;
pub mod lwe;
pub mod security;
pub mod stats;
pub mod zk;

pub use hash_commit::*;
pub use hybrid_commit::*;
pub use lattice_commit::*;
pub use lwe::*;
pub use security::*;
pub use stats::*;
pub use zk::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lwe::PublicParams;
    use crate::security::decryption_failure_bound_per_coord;
    use crate::stats::calculate_stats;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    fn setup_pp() -> (PublicParams, StdRng) {
        let mut rng = StdRng::seed_from_u64(42);
        let n = 256;
        let q = 12289;
        let pp = PublicParams::generate(n, q, &mut rng);
        (pp, rng)
    }

    #[test]
    fn test_hybrid_commit_verify_roundtrip() {
        let (pp, mut rng) = setup_pp();
        let msg = b"buy 100 EURUSD";
        let (commit, opening) = hybrid_commit(&pp, msg, 3.2, &mut rng);
        assert!(hybrid_verify(&pp, &commit, &opening));
    }

    #[test]
    fn test_hybrid_verify_rejects_wrong_message() {
        let (pp, mut rng) = setup_pp();
        let msg = b"buy 100 EURUSD";
        let (commit, mut opening) = hybrid_commit(&pp, msg, 3.2, &mut rng);
        opening.message = b"sell 100 EURUSD".to_vec();
        assert!(!hybrid_verify(&pp, &commit, &opening));
    }

    #[test]
    fn test_hybrid_verify_rejects_wrong_opening() {
        let (pp, mut rng) = setup_pp();
        let msg = b"buy 100 EURUSD";
        let (commit, mut opening) = hybrid_commit(&pp, msg, 3.2, &mut rng);

        // Tamper with r
        opening.r[0] = (opening.r[0] + 1) % pp.q;
        assert!(!hybrid_verify(&pp, &commit, &opening));

        // Fix r, tamper with e
        opening.r[0] = (opening.r[0] + pp.q - 1) % pp.q;
        opening.e[0] += 1;
        assert!(!hybrid_verify(&pp, &commit, &opening));
    }

    #[test]
    fn test_hash_check_short_circuits() {
        // We will mock the verify function's short-circuit behavior by showing that
        // a wrong C1 fails quickly without needing the C2 check. We can't easily count
        // instructions here without a profiler, but we can verify it fails correctly
        // when C1 is wrong but C2 is right (which is what short circuiting achieves).
        let (pp, mut rng) = setup_pp();
        let msg = b"buy 100 EURUSD";
        let (mut commit, opening) = hybrid_commit(&pp, msg, 3.2, &mut rng);

        commit.c1[0] ^= 1; // flip a bit in the hash
        assert!(!hybrid_verify(&pp, &commit, &opening));
    }

    #[test]
    fn test_encode_injective() {
        let n = 256;
        let q = 12289;
        let msg1 = b"short message 1";
        let msg2 = b"short message 2";
        let encoded1 = encode(msg1, n, q);
        let encoded2 = encode(msg2, n, q);
        assert_ne!(encoded1, encoded2);
    }

    #[test]
    fn test_zk_completeness() {
        let (pp, mut rng) = setup_pp();
        let msg = b"buy 100 EURUSD";
        let (commit, opening) = hybrid_commit(&pp, msg, 3.2, &mut rng);
        let proof = zk_prove(&pp, &commit, msg, &opening.r, &opening.e, 10.0, &mut rng);
        assert!(zk_verify(&pp, &commit, &proof, msg));
    }

    #[test]
    fn test_zk_rejects_wrong_witness() {
        let (pp, mut rng) = setup_pp();
        let msg = b"buy 100 EURUSD";
        let (commit, mut opening) = hybrid_commit(&pp, msg, 3.2, &mut rng);

        // Wrong witness
        opening.r[0] = (opening.r[0] + 1) % pp.q;

        // Need to do this over multiple random challenges if we were interactive,
        // but Fiat-Shamir means the proof is tied to the statement.
        // If we generate a proof with a wrong witness, it should fail verify.
        // Actually, proving with a wrong witness might still pass the algebraic check
        // IF we don't fix the challenge. Wait, zk_prove uses the passed in r and e
        // to form the response. If r and e don't match C2, the response will be invalid
        // for the challenge.
        let proof = zk_prove(&pp, &commit, msg, &opening.r, &opening.e, 10.0, &mut rng);
        assert!(!zk_verify(&pp, &commit, &proof, msg));
    }

    #[test]
    fn test_decryption_failure_bound_monotone() {
        let sigma = 3.2;
        let bound1 = decryption_failure_bound_per_coord(sigma, 1.0);
        let bound2 = decryption_failure_bound_per_coord(sigma, 2.0);
        let bound3 = decryption_failure_bound_per_coord(sigma, 3.0);

        assert!(bound1 > bound2);
        assert!(bound2 > bound3);
    }

    #[test]
    fn test_confidence_interval() {
        let mut rng = StdRng::seed_from_u64(42);

        let mut large_sample = vec![];
        for _ in 0..1000 {
            large_sample.push(rng.gen_range(0.0..10.0));
        }

        let mut small_sample = vec![];
        for _ in 0..10 {
            small_sample.push(rng.gen_range(0.0..10.0));
        }

        let stats_large = calculate_stats(large_sample);
        let stats_small = calculate_stats(small_sample);

        let width_large = stats_large.ci_upper_ms - stats_large.ci_lower_ms;
        let width_small = stats_small.ci_upper_ms - stats_small.ci_lower_ms;

        assert!(width_large < width_small);
    }
}
