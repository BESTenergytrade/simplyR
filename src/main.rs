use std::env;
use std::fs::File;
use std::io::BufReader;

use serde::{Deserialize, Serialize};

/// Smallest energy value (in kWh) that is used for a match.
const ENERGY_EPS: f64 = 0.001;

fn round_energy_value(energy: f64) -> f64 {
    (energy * 1000.0).round() / 1000.0
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
enum OrderType {
    #[serde(rename = "bid")]
    Bid,
    #[serde(rename = "ask")]
    Ask,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Order {
    id: u64,
    order_type: OrderType,
    time_slot: String,
    actor_id: String,
    cluster_index: i64,
    energy_kwh: f64,
    price_euro_per_kwh: f64,
}

/// The market input contains all orders of a time slot.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct MarketInput {
    orders: Vec<Order>,
}

/// A match between a bid and an ask.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Match {
    bid_id: u64,
    ask_id: u64,
    energy_kwh: f64,
    price_euro_per_kwh: f64,
}

/// The market output contains all matches of a time slot.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct MarketOutput {
    matches: Vec<Match>,
}

fn pay_as_bid_matching(input: &MarketInput) -> MarketOutput {
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

fn example_code() {
    // Create some orders
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
        energy_kwh: 1.5,
        price_euro_per_kwh: 0.35,
    };

    // Gather orders into a MarketInput struct
    let market_input = MarketInput {
        orders: vec![order_1, order_2],
    };

    // Serialize to a JSON string
    let serialized = serde_json::to_string(&market_input).unwrap();
    println!("As JSON:\n\n{}\n", serialized);

    // Deserialized from a JSON string
    let deserialized: MarketInput = serde_json::from_str(&serialized).unwrap();
    println!("As struct:\n\n{:#?}\n", deserialized);

    // Match
    let market_output = pay_as_bid_matching(&market_input);
    println!("Matched:\n\n{:#?}", market_output);
}

fn print_usage() {
    println!(
        "Take orders from a JSON file and print matches as JSON\n\n\
    Usage:
      cargo run -- example_market_input.json

      or

      cargo build --release
      target/release/rust-matching example_market_input.json
    "
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        // first commandline argument is json file path
        let file = File::open(&args[1])?;
        let reader = BufReader::new(file);
        let market_input: MarketInput = serde_json::from_reader(reader)?;
        let market_output = pay_as_bid_matching(&market_input);
        let market_output_json = serde_json::to_string_pretty(&market_output).unwrap();
        println!("{}", market_output_json);
    } else {
        print_usage();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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

            println!("{:#?}", market_output);

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
