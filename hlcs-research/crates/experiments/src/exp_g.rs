use forex_sim::{generate_price_series, Tick};
use rand::{rngs::StdRng, SeedableRng};

pub struct ExpGResult {
    pub ticks: Vec<Tick>,
}

pub fn run(seed: u64, num_ticks: usize) -> ExpGResult {
    let mut rng = StdRng::seed_from_u64(seed);
    let ticks = generate_price_series(1.1000, 0.0001, 10.0, num_ticks, &mut rng);
    ExpGResult { ticks }
}
