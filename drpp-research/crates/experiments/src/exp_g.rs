use crate::config::Config;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_distr::{Distribution, Poisson};

#[derive(Debug)]
pub struct ResultG {
    pub time_s: u32,
    pub requests_no_rl: u32,
    pub requests_with_rl: u32,
    pub blocked: u32,
}

pub fn run(cfg: &Config) -> anyhow::Result<Vec<ResultG>> {
    let mut rng = StdRng::seed_from_u64(cfg.seed + 777);
    let lambda = 5.0; // Poisson(λ=5) arrivals per second
    let dist = Poisson::new(lambda).unwrap();

    let rate_limit = 2; // Allow max 2 requests per second
    let duration_s = 60; // Simulate 60 seconds

    let mut results = Vec::with_capacity(duration_s as usize);

    let mut cum_no_rl = 0;
    let mut cum_with_rl = 0;
    let mut cum_blocked = 0;

    for t in 1..=duration_s {
        let arrivals = dist.sample(&mut rng) as u32;
        let allowed = std::cmp::min(arrivals, rate_limit);
        let blocked = arrivals.saturating_sub(rate_limit);

        cum_no_rl += arrivals;
        cum_with_rl += allowed;
        cum_blocked += blocked;

        results.push(ResultG {
            time_s: t,
            requests_no_rl: cum_no_rl,
            requests_with_rl: cum_with_rl,
            blocked: cum_blocked,
        });
    }

    Ok(results)
}
