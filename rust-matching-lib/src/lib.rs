#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

/// Smallest energy value (in kWh) that is used for a match.
const ENERGY_EPS: f64 = 0.001;

fn round_energy_value(energy: f64) -> f64 {
    (energy * 1000.0).round() / 1000.0
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum OrderType {
    #[serde(rename = "bid")]
    Bid,
    #[serde(rename = "ask")]
    Ask,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: u64,
    pub order_type: OrderType,
    pub time_slot: String,
    pub actor_id: String,
    pub cluster_index: i64,
    pub energy_kwh: f64,
    pub price_euro_per_kwh: f64,
}

/// The market input contains all orders of a time slot.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MarketInput {
    pub orders: Vec<Order>,
}

/// A match between a bid and an ask.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Match {
    pub bid_id: u64,
    pub ask_id: u64,
    pub energy_kwh: f64,
    pub price_euro_per_kwh: f64,
}

/// The market output contains all matches of a time slot.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MarketOutput {
    pub matches: Vec<Match>,
}

pub fn pay_as_bid_matching(input: &MarketInput) -> MarketOutput {
    let mut bids: Vec<Order> = vec![];
    let mut asks: Vec<Order> = vec![];

    // Gather bids and asks
    for order in input.orders.iter().cloned() {
        match order.order_type {
            OrderType::Bid => {
                bids.push(order);
            }
            OrderType::Ask => {
                asks.push(order);
            }
        }
    }

    let mut matches = vec![];

    // Sort by price
    bids.sort_by(|a, b| {
        a.price_euro_per_kwh
            .total_cmp(&b.price_euro_per_kwh)
            .reverse()
    });
    asks.sort_by(|a, b| a.price_euro_per_kwh.total_cmp(&b.price_euro_per_kwh));

    // Make bids immutable to avoid accidentally changing them
    let bids = bids;

    // match
    for bid in &bids {
        let mut remaining_energy = bid.energy_kwh;
        for ask in asks.iter_mut() {
            if (bid.price_euro_per_kwh >= ask.price_euro_per_kwh) && (ask.energy_kwh > ENERGY_EPS) {
                let matched_energy = ask.energy_kwh.min(remaining_energy);
                matches.push(Match {
                    bid_id: bid.id,
                    ask_id: ask.id,
                    energy_kwh: round_energy_value(matched_energy),
                    price_euro_per_kwh: bid.price_euro_per_kwh,
                });
                ask.energy_kwh -= matched_energy;
                remaining_energy -= matched_energy;
                if remaining_energy < ENERGY_EPS {
                    break;
                }
            }
        }
    }

    MarketOutput { matches }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::alloc::string::ToString;

    #[test]
    fn test_pay_as_bid() {
        {
            let order_1 = Order {
                id: 1,
                order_type: OrderType::Ask,
                time_slot: "2022-03-04T05:06:07+00:00".to_string(),
                actor_id: "actor_1".to_string(),
                cluster_index: 0,
                energy_kwh: 2.0,
                price_euro_per_kwh: 0.30,
            };

            let order_2 = Order {
                id: 2,
                order_type: OrderType::Bid,
                time_slot: "2022-03-04T05:06:07+00:00".to_string(),
                actor_id: "actor_2".to_string(),
                cluster_index: 0,
                energy_kwh: 2.0,
                price_euro_per_kwh: 0.30,
            };

            let market_input = MarketInput {
                orders: vec![order_1, order_2],
            };

            let market_output = pay_as_bid_matching(&market_input);

            assert_eq!(market_output.matches.len(), 1);
            let m = &market_output.matches[0];
            assert_eq!(m.energy_kwh, 2.0);
            assert_eq!(m.price_euro_per_kwh, 0.3);
        }

        {
            let order_1 = Order {
                id: 1,
                order_type: OrderType::Ask,
                time_slot: "2022-03-04T05:06:07+00:00".to_string(),
                actor_id: "actor_1".to_string(),
                cluster_index: 0,
                energy_kwh: 3.0,
                price_euro_per_kwh: 0.30,
            };

            let order_2 = Order {
                id: 2,
                order_type: OrderType::Bid,
                time_slot: "2022-03-04T05:06:07+00:00".to_string(),
                actor_id: "actor_2".to_string(),
                cluster_index: 0,
                energy_kwh: 2.0,
                price_euro_per_kwh: 0.40,
            };

            let order_3 = Order {
                id: 3,
                order_type: OrderType::Bid,
                time_slot: "2022-03-04T05:06:07+00:00".to_string(),
                actor_id: "actor_3".to_string(),
                cluster_index: 0,
                energy_kwh: 2.0,
                price_euro_per_kwh: 0.30,
            };

            let market_input = MarketInput {
                orders: vec![order_1, order_2, order_3],
            };

            let market_output = pay_as_bid_matching(&market_input);

            assert_eq!(market_output.matches.len(), 2);
            let m1 = &market_output.matches[0];
            assert_eq!(m1.energy_kwh, 2.0);
            assert_eq!(m1.price_euro_per_kwh, 0.4);
            let m2 = &market_output.matches[1];
            assert_eq!(m2.energy_kwh, 1.0);
            assert_eq!(m2.price_euro_per_kwh, 0.3);
        }

        {
            let order_1 = Order {
                id: 1,
                order_type: OrderType::Ask,
                time_slot: "2022-03-04T05:06:07+00:00".to_string(),
                actor_id: "actor_1".to_string(),
                cluster_index: 0,
                energy_kwh: 3.0,
                price_euro_per_kwh: 0.20,
            };

            let order_2 = Order {
                id: 2,
                order_type: OrderType::Ask,
                time_slot: "2022-03-04T05:06:07+00:00".to_string(),
                actor_id: "actor_2".to_string(),
                cluster_index: 0,
                energy_kwh: 2.0,
                price_euro_per_kwh: 0.25,
            };

            let order_3 = Order {
                id: 3,
                order_type: OrderType::Bid,
                time_slot: "2022-03-04T05:06:07+00:00".to_string(),
                actor_id: "actor_3".to_string(),
                cluster_index: 0,
                energy_kwh: 4.0,
                price_euro_per_kwh: 0.30,
            };

            let market_input = MarketInput {
                orders: vec![order_1, order_2, order_3],
            };

            let market_output = pay_as_bid_matching(&market_input);

            assert_eq!(market_output.matches.len(), 2);
            let m1 = &market_output.matches[0];
            assert_eq!(m1.energy_kwh, 3.0);
            assert_eq!(m1.price_euro_per_kwh, 0.3);
            let m2 = &market_output.matches[1];
            assert_eq!(m2.energy_kwh, 1.0);
            assert_eq!(m2.price_euro_per_kwh, 0.3);
        }
    }
}
