use rand::Rng;
use rand_distr::{Distribution, Normal};

pub fn generate_touch_features<R: Rng>(is_legit: bool, noise_level: f64, rng: &mut R) -> Vec<f64> {
    let mut features = Vec::with_capacity(12);

    // Base parameters
    let (cap_mean, cap_std) = if is_legit {
        (150.0, 20.0)
    } else {
        (200.0, 5.0)
    };
    let (area_mean, area_std) = if is_legit { (2.5, 0.5) } else { (3.5, 0.1) };

    // Apply noise
    let cap_std = cap_std * (1.0 + noise_level);
    let area_std = area_std * (1.0 + noise_level);

    let cap_dist = Normal::new(cap_mean, cap_std).unwrap();
    let area_dist = Normal::new(area_mean, area_std).unwrap();

    // 8 capacitance features
    for _ in 0..8 {
        features.push(cap_dist.sample(rng));
    }

    // 4 area features
    for _ in 0..4 {
        features.push(area_dist.sample(rng));
    }

    features
}
