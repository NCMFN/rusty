use rand::Rng;
use rand_distr::{Distribution, Normal};

pub fn generate_knock_features<R: Rng>(is_legit: bool, noise_level: f64, rng: &mut R) -> Vec<f64> {
    let mut features = Vec::with_capacity(13);

    // Base parameters
    let (timing_mean, timing_std) = if is_legit {
        (300.0, 40.0)
    } else {
        (300.0, 3.0)
    };
    let (force_mean, force_std) = if is_legit { (0.6, 0.15) } else { (0.6, 0.02) };
    let (rhythm_mean, rhythm_std) = if is_legit { (1.0, 0.12) } else { (1.0, 0.02) };

    // Apply noise
    let timing_std = timing_std * (1.0 + noise_level);
    let force_std = force_std * (1.0 + noise_level);
    let rhythm_std = rhythm_std * (1.0 + noise_level);

    let timing_dist = Normal::new(timing_mean, timing_std).unwrap();
    let force_dist = Normal::new(force_mean, force_std).unwrap();
    let rhythm_dist = Normal::new(rhythm_mean, rhythm_std).unwrap();

    // 5 timing features
    for _ in 0..5 {
        features.push(timing_dist.sample(rng));
    }

    // 5 force features
    for _ in 0..5 {
        features.push(force_dist.sample(rng));
    }

    // 3 rhythm features
    for _ in 0..3 {
        features.push(rhythm_dist.sample(rng));
    }

    features
}
