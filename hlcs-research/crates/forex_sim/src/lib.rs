pub mod burst;
pub mod orders;
pub mod price_model;

pub use burst::*;
pub use orders::*;
pub use price_model::*;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn test_order_generation_counts() {
        let mut rng = StdRng::seed_from_u64(42);
        let n_traders = 50;
        let orders_per_trader = 10;
        let orders = generate_orders(
            n_traders,
            orders_per_trader,
            1000.0,
            100_000.0,
            1000.0,
            &mut rng,
        );

        assert_eq!(orders.len(), n_traders * orders_per_trader);
        for order in &orders {
            assert!(order.notional_usd >= 1000.0);
            assert!(order.notional_usd < 100_000.0);
        }
    }

    #[test]
    fn test_price_series_reproducible() {
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);

        let series1 = generate_price_series(1.1000, 0.0001, 10.0, 100, &mut rng1);
        let series2 = generate_price_series(1.1000, 0.0001, 10.0, 100, &mut rng2);

        for i in 0..100 {
            assert_eq!(series1[i].price, series2[i].price);
        }
    }
}
