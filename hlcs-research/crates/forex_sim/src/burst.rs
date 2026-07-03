pub struct BurstResult {
    pub target_orders: u64,
    pub orders_processed: u64,
    pub orders_dropped: u64,
    pub actual_throughput: f64,
}

/// Simulates processing a burst of orders to see how many can be processed within
/// a strict latency window (e.g., 1 ms).
/// `target_orders_per_sec`: The arrival rate of orders.
/// `window_ms`: The total time window we are simulating (e.g. 1.0 ms).
/// `per_order_latency_ns`: Average time to process a single order in nanoseconds.
pub fn simulate_burst(
    target_orders_per_sec: u64,
    window_ms: f64,
    per_order_latency_ns: f64,
) -> BurstResult {
    // How many orders arrive in this window?
    let window_secs = window_ms / 1000.0;
    let target_orders = (target_orders_per_sec as f64 * window_secs) as u64;

    // How much time do we actually have to process?
    let window_ns = window_ms * 1_000_000.0;

    // How many can we process sequentially in this window?
    // Assuming a single-threaded queue for the burst processor.
    let max_processed = (window_ns / per_order_latency_ns).floor() as u64;

    let orders_processed = std::cmp::min(target_orders, max_processed);
    let orders_dropped = target_orders.saturating_sub(orders_processed);

    let actual_throughput = if orders_processed > 0 {
        (orders_processed as f64) / window_secs
    } else {
        0.0
    };

    BurstResult {
        target_orders,
        orders_processed,
        orders_dropped,
        actual_throughput,
    }
}
