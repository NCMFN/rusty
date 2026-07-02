use rand::Rng;
use rand_distr::{Distribution, Normal};

pub fn generate_gesture_features<R: Rng>(
    is_legit: bool,
    noise_level: f64,
    rng: &mut R,
) -> Vec<f64> {
    let mut features = Vec::with_capacity(21);

    // Base parameters
    let xyz_mean = 0.0;
    let xyz_std = 1.0 * (1.0 + noise_level);

    let vel_mean = 0.5;
    let vel_std = 0.2 * (1.0 + noise_level);

    let depth_mean = if is_legit { 0.4 } else { 0.0 };
    let depth_std = if is_legit { 0.1 } else { 0.01 };
    let depth_std = depth_std * (1.0 + noise_level);

    let xyz_dist = Normal::new(xyz_mean, xyz_std).unwrap();
    let vel_dist = Normal::new(vel_mean, vel_std).unwrap();
    let depth_dist = Normal::new(depth_mean, depth_std).unwrap();

    // 12 xyz features
    for _ in 0..12 {
        features.push(xyz_dist.sample(rng));
    }

    // 6 velocity features
    for _ in 0..6 {
        features.push(vel_dist.sample(rng));
    }

    // 3 depth features
    for _ in 0..3 {
        features.push(depth_dist.sample(rng));
    }

    features
}
