use rand::rngs::StdRng;
use rand_distr::{Distribution, Normal};

#[derive(Clone, Debug)]
pub struct Tick {
    pub time_ms: f64,
    pub price: f64,
}

/// Generates a stochastic EUR/USD mid-price tick series using a random walk.
/// Replaces the mislabeled heatmap figure in the paper.
pub fn generate_price_series(
    start_price: f64,
    volatility: f64, // e.g., 0.0001
    dt_ms: f64,      // e.g., 10.0 ms per tick
    num_ticks: usize,
    rng: &mut StdRng,
) -> Vec<Tick> {
    let mut ticks = Vec::with_capacity(num_ticks);
    let mut current_price = start_price;
    let mut current_time = 0.0;

    let normal = Normal::new(0.0, volatility).unwrap();

    for _ in 0..num_ticks {
        ticks.push(Tick {
            time_ms: current_time,
            price: current_price,
        });

        let step = normal.sample(rng);
        current_price += step;
        current_time += dt_ms;
    }

    ticks
}
