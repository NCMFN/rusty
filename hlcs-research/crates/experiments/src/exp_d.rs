use forex_sim::{format_order_message, generate_orders};
use hlcs_core::{calculate_stats, hybrid_commit, LatencyStats, PublicParams};
use rand::{rngs::StdRng, SeedableRng};
use std::collections::HashMap;
use std::time::Instant;

pub struct TraderSummary {
    pub trader_id: usize,
    pub num_orders: usize,
    pub mean_latency_ms: f64,
    pub total_volume_usd: f64,
}

pub struct ExpDResult {
    pub all_latencies_ms: Vec<f64>,
    pub overall_stats: LatencyStats,
    pub trader_summaries: Vec<TraderSummary>,
}

pub fn run(
    seed: u64,
    n_traders: usize,
    orders_per_trader: usize,
    n: usize,
    q: u32,
    sigma: f64,
) -> ExpDResult {
    let mut rng = StdRng::seed_from_u64(seed);
    let pp = PublicParams::generate(n, q, &mut rng);

    // Simulate 1000ms window
    let orders = generate_orders(
        n_traders,
        orders_per_trader,
        1000.0,
        100_000.0,
        1000.0,
        &mut rng,
    );

    let mut all_latencies_ms = Vec::with_capacity(orders.len());
    let mut latencies_by_trader: HashMap<usize, Vec<f64>> = HashMap::new();
    let mut volume_by_trader: HashMap<usize, f64> = HashMap::new();

    for order in &orders {
        let msg = format_order_message(order);

        let start = Instant::now();
        let _ = hybrid_commit(&pp, &msg, sigma, &mut rng);
        let latency_ms = start.elapsed().as_nanos() as f64 / 1_000_000.0;

        all_latencies_ms.push(latency_ms);

        latencies_by_trader
            .entry(order.trader_id)
            .or_default()
            .push(latency_ms);

        *volume_by_trader.entry(order.trader_id).or_insert(0.0) += order.notional_usd;
    }

    let overall_stats = calculate_stats(all_latencies_ms.clone());

    let mut trader_summaries = Vec::new();
    for (&trader_id, lats) in &latencies_by_trader {
        let stats = calculate_stats(lats.clone());
        trader_summaries.push(TraderSummary {
            trader_id,
            num_orders: lats.len(),
            mean_latency_ms: stats.mean_ms,
            total_volume_usd: *volume_by_trader.get(&trader_id).unwrap_or(&0.0),
        });
    }

    trader_summaries.sort_by_key(|t| t.trader_id);

    ExpDResult {
        all_latencies_ms,
        overall_stats,
        trader_summaries,
    }
}
