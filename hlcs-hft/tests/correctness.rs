use hlcs_hft::*;
use proptest::prelude::*;
use rand::{rngs::StdRng, SeedableRng};

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn test_commit_verify_honest(
        msg in prop::collection::vec(any::<u8>(), 0..1024),
        seed in any::<u64>()
    ) {
        let mut rng = StdRng::seed_from_u64(seed);
        let n = 256;
        let pp = PublicParams::new(n, &mut rng);
        let (c, hint) = commit(&pp, &msg, &mut rng, SIGMA);
        assert!(verify(&pp, &c, &hint));
    }

    #[test]
    fn test_tampered_commitment(
        msg in prop::collection::vec(any::<u8>(), 0..1024),
        seed in any::<u64>()
    ) {
        let mut rng = StdRng::seed_from_u64(seed);
        let n = 256;
        let pp = PublicParams::new(n, &mut rng);
        let (mut c, hint) = commit(&pp, &msg, &mut rng, SIGMA);
        c.c1[0] ^= 1; // Tamper
        assert!(!verify(&pp, &c, &hint));
    }
}
