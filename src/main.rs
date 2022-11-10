use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum OrderType {
    #[serde(rename = "bid")]
    Bid,
    #[serde(rename = "ask")]
    Ask,
}

#[derive(Debug, Serialize, Deserialize)]
struct Order {
    order_type: OrderType,
    time_slot: String,
    actor_id: String,
    cluster_index: i64,
    energy_kwh: f64,
    price_euro_per_kwh: f64,
}

/// The market input contains all orders of a time slot.
#[derive(Debug, Serialize, Deserialize)]
struct MarketInput {
    orders: Vec<Order>,
}

fn main() {
    let order_1 = Order {
        order_type: OrderType::Ask,
        time_slot: "2022-03-04T05:06:07+00:00".to_string(),
        actor_id: "actor_1".to_string(),
        cluster_index: 0,
        energy_kwh: 2.0,
        price_euro_per_kwh: 0.30,
    };

    let order_2 = Order {
        order_type: OrderType::Bid,
        time_slot: "2022-03-04T05:06:07+00:00".to_string(),
        actor_id: "actor_1".to_string(),
        cluster_index: 0,
        energy_kwh: 2.0,
        price_euro_per_kwh: 0.30,
    };

    let market_input = MarketInput {
        orders: vec![order_1, order_2],
    };

    // Serialize to a string
    let json_str = serde_json::to_string(&market_input).unwrap();
    println!("As JSON:\n{}", json_str);

    // Deserialized from a JSON string
    let deserialized: MarketInput = serde_json::from_str(&json_str).unwrap();
    println!("\nAs struct:\n{:#?}", deserialized);
}
