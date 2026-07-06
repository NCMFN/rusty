use rand::rngs::StdRng;
use rand::RngCore;
use sha3::{Digest, Sha3_256};

/// A simple hash-only commitment baseline using SHA3-256.
/// Commit(m, r) = SHA3-256(m || r)
#[derive(Clone, Debug, PartialEq)]
pub struct HashCommitment {
    pub hash: [u8; 32],
}

pub struct HashOpening {
    pub message: Vec<u8>,
    pub r: [u8; 32],
}

pub fn hash_commit(message: &[u8], rng: &mut StdRng) -> (HashCommitment, HashOpening) {
    let mut r = [0u8; 32];
    rng.fill_bytes(&mut r);

    let mut hasher = Sha3_256::new();
    hasher.update(message);
    hasher.update(&r);
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);

    (
        HashCommitment { hash },
        HashOpening {
            message: message.to_vec(),
            r,
        },
    )
}

pub fn hash_verify(commitment: &HashCommitment, opening: &HashOpening) -> bool {
    let mut hasher = Sha3_256::new();
    hasher.update(&opening.message);
    hasher.update(&opening.r);
    let result = hasher.finalize();

    let mut expected_hash = [0u8; 32];
    expected_hash.copy_from_slice(&result);

    commitment.hash == expected_hash
}
