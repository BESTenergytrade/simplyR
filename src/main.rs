use std::env;
use std::fs::File;
use std::io::BufReader;

use simplyr_lib::{custom_fair_matching, pay_as_bid_matching, MarketInput, Order, OrderType};

#[allow(dead_code)]
fn example_code() {
    // Create some orders
    let order_1 = Order {
        id: 1,
        order_type: OrderType::Ask,
        time_slot: "2022-03-04T05:06:07+00:00".to_string(),
        actor_id: "actor_1".to_string(),
        cluster_index: Some(0),
        energy_kwh: 2.0,
        price_euro_per_kwh: 0.30,
    };

    let order_2 = Order {
        id: 2,
        order_type: OrderType::Bid,
        time_slot: "2022-03-04T05:06:07+00:00".to_string(),
        actor_id: "actor_2".to_string(),
        cluster_index: Some(0),
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
    let fair_market_output = custom_fair_matching(&market_input);
    let pay_as_bid_market_output = pay_as_bid_matching(&market_input);
    println!("Matched fair:\n\n{:#?}", fair_market_output);
    println!("Matched pay as bid:\n\n{:#?}", pay_as_bid_market_output);
}

fn print_usage() {
    println!(
        "Take orders from a JSON file and print matches as JSON\n\n\
    Usage:
      cargo run -- example_market_input.json

      or

      cargo build --release
      target/release/simplyr example_market_input.json
    "
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    example_code();
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        // first commandline argument is json file path
        let file = File::open(&args[1])?;
        let reader = BufReader::new(file);
        let market_input: MarketInput = serde_json::from_reader(reader)?;

        {
            let market_output = custom_fair_matching(&market_input);
            let market_output_json = serde_json::to_string_pretty(&market_output)?;
            println!("Fair:\n{}\n", market_output_json);
        }

        {
            let market_output = pay_as_bid_matching(&market_input);
            let market_output_json = serde_json::to_string_pretty(&market_output)?;
            println!("Pay as bid:\n{}\n", market_output_json);
        }
    } else {
        print_usage();
    }

    Ok(())
}
