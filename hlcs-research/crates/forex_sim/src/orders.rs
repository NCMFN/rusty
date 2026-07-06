use rand::rngs::StdRng;
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub enum OrderDirection {
    Buy,
    Sell,
}

#[derive(Clone, Debug)]
pub struct Order {
    pub trader_id: usize,
    pub order_id: usize,
    pub direction: OrderDirection,
    pub notional_usd: f64,
    pub arrival_time_ms: f64,
    pub price: f64,
}

/// Generates the simulated forex workload:
/// 50 traders x 10 orders = 500 orders
/// Random notional in [$1,000, $100,000]
/// Random buy/sell direction
/// Random timestamps
pub fn generate_orders(
    n_traders: usize,
    orders_per_trader: usize,
    min_notional: f64,
    max_notional: f64,
    time_window_ms: f64,
    rng: &mut StdRng,
) -> Vec<Order> {
    let total_orders = n_traders * orders_per_trader;
    let mut orders = Vec::with_capacity(total_orders);

    for trader_id in 0..n_traders {
        for order_id in 0..orders_per_trader {
            let direction = if rng.gen_bool(0.5) {
                OrderDirection::Buy
            } else {
                OrderDirection::Sell
            };

            let notional_usd = rng.gen_range(min_notional..max_notional);
            let arrival_time_ms = rng.gen_range(0.0..time_window_ms);

            // The exact price doesn't matter for the commitment payload latency,
            // we just generate a plausible message string.
            let price = 1.1000 + rng.gen_range(-0.01..0.01);

            orders.push(Order {
                trader_id,
                order_id,
                direction,
                notional_usd,
                arrival_time_ms,
                price,
            });
        }
    }

    // Sort by arrival time
    orders.sort_by(|a, b| a.arrival_time_ms.partial_cmp(&b.arrival_time_ms).unwrap());
    orders
}

pub fn format_order_message(order: &Order) -> Vec<u8> {
    let dir_str = match order.direction {
        OrderDirection::Buy => "BUY",
        OrderDirection::Sell => "SELL",
    };
    format!(
        "{} {} EURUSD @ {:.4} T{}",
        dir_str, order.notional_usd as u64, order.price, order.trader_id
    )
    .into_bytes()
}
