use crate::protocol::{compute_response, generate_challenge, verify};
use rand::rngs::StdRng;
use rand::Rng;

/// Simulates a single random guess attack.
/// Returns true if the adversary wins.
pub fn single_guess_attack(k: u32, secret: &[u8], rng: &mut StdRng) -> bool {
    let challenge = generate_challenge(k, rng);
    let guess = generate_challenge(k, rng);
    verify(challenge, guess, secret, k)
}

/// Simulates a collusion attack with n colluders.
/// Each colluder makes one random guess, they share information to avoid duplicates.
/// Returns true if the adversary wins.
pub fn collusion_attack(k: u32, n: usize, secret: &[u8], rng: &mut StdRng) -> bool {
    let challenge = generate_challenge(k, rng);
    let expected = compute_response(challenge, secret, k);

    // Simplest simulation: the probability of success is n / 2^k,
    // assuming they don't guess the same thing.
    // If n >= 2^k, they always win.

    let space_size = 1u64 << k;
    if (n as u64) >= space_size {
        return true;
    }

    // We simulate by drawing n without replacement
    // Since n is small, we can just reject duplicates, or use a set.
    // Or simpler: generate n random guesses. If any is correct, win.
    // For exact match of "they share info to avoid duplicates", we can do:
    let mut guessed = std::collections::HashSet::new();
    for _ in 0..n {
        let mut guess;
        loop {
            guess = generate_challenge(k, rng);
            if !guessed.contains(&guess) {
                break;
            }
        }
        guessed.insert(guess);
        if guess == expected {
            return true;
        }
    }

    false
}

/// Simulates a "traditional" authentication attack based on ambient cues.
/// Success is determined by a fixed deception probability.
pub fn traditional_attack(deception_prob: f64, rng: &mut StdRng) -> bool {
    rng.gen::<f64>() < deception_prob
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn test_collusion_monotone() {
        let secret = b"secret";
        let k = 8;
        let trials = 1000;
        let mut rng = StdRng::seed_from_u64(42);

        let mut p_prev = 0.0;
        for n in [1, 2, 4, 8, 16] {
            let mut wins = 0;
            for _ in 0..trials {
                if collusion_attack(k, n, secret, &mut rng) {
                    wins += 1;
                }
            }
            let p = wins as f64 / trials as f64;
            assert!(p >= p_prev || (p_prev - p) < 0.05); // Monotonic with some margin for noise
            p_prev = p;
        }
    }
}
